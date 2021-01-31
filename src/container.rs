/*
   Copyright 2021 Alexander Efremkin

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use crate::vector::Vector;
use crate::particle::{MovingParticle, StaticParticle};
use std::slice;

/// Reference to a particle of a certain type
pub struct ParticleRef<'a, T> {
    pub particle: &'a T,
    pub index: usize,
}
/// Copied reference, used when a keeping a reference
/// is not convenient. These are usually merged back
/// with "update" container method or discarded.
pub struct Particle<T>
where
    T: Copy,
{
    pub particle: T,
    pub index: usize,
}

impl<'a, T> ParticleRef<'a, T>
where
    T: Copy,
{
    /// Create a particle copy from reference,
    /// which still (logically) denotes a particle in a container
    pub fn as_copy(&self) -> Particle<T> {
        Particle::<T> {
            particle: *self.particle,
            index: self.index,
        }
    }
}

/// Container for particles
pub struct ParticleContainer<T>
where
    T: Copy,
{
    particles: Vec<T>,
}

pub type MovingParticleContainer = ParticleContainer<MovingParticle>;
pub type StaticParticleContainer = ParticleContainer<StaticParticle>;

pub struct ContainerIterator<'a, T> {
    iter: std::iter::Enumerate<slice::Iter<'a, T>>,
}

impl<'a, T> Iterator for ContainerIterator<'a, T> {
    type Item = ParticleRef<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, particle)| ParticleRef { particle, index })
    }
}

impl<T> ParticleContainer<T>
where
    T: Copy,
{
    /// Create a new container with specified capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            particles: Vec::with_capacity(capacity),
        }
    }

    /// Check if container cannot have any more particles
    pub fn is_full(&self) -> bool {
        self.particles.len() == self.particles.capacity()
    }

    /// Return current number of particles
    pub fn size(&self) -> usize {
        self.particles.len()
    }
    /// Return maximum number of particles
    pub fn max_size(&self) -> usize {
        self.particles.capacity()
    }

    /// Provide an iterator of ParticleRef's over the container
    pub fn values(&self) -> ContainerIterator<T> {
        ContainerIterator {
            iter: self.particles.iter().enumerate(),
        }
    }

    /// Apply an operator to particles
    pub fn apply<F>(&mut self, operator: F)
    where
        F: std::ops::Fn(&mut T) -> (),
    {
        self.particles.iter_mut().for_each(operator);
    }

    /// Add a new particle to the container
    pub fn add_particle(&mut self, particle: T) -> Option<()> {
        if self.is_full() {
            None
        } else {
            self.particles.push(particle);
            Some(())
        }
    }

    /// Get a reference to particle at an index
    pub fn at(&self, index: usize) -> Option<&T> {
        if index < self.particles.len() {
            Some(&self.particles[index])
        } else {
            None
        }
    }

    /// Remove one particle using its index
    pub fn remove_one_by_index(&mut self, index: usize) {
        if index < self.particles.len() {
            self.particles.swap_remove(index);
        }
    }

    /// Remove multiple particles using their indices
    pub fn remove_multiple_by_index(&mut self, mut items: Vec<usize>) {
        // We have to remove items in backwards order to successfully process
        // all of them.
        items.sort_unstable_by_key(|&k| std::cmp::Reverse(k));
        let own_size = self.particles.len();
        for index in items.iter().skip_while(|i| **i >= own_size) {
            // we can use swap_remove to avoid copying the vector tail over and over
            self.particles.swap_remove(*index);
        }
    }

    /// Get pointer to a contiguous container memory area
    pub fn as_ptr(&self) -> *const T {
        self.particles.as_ptr()
    }

    /// Update a particle (from an copied reference)
    pub fn update(&mut self, particle: &Particle<T>) {
        if particle.index < self.particles.len() {
            self.particles[particle.index] = particle.particle
        }
    }
}

impl StaticParticleContainer {
    pub fn select_for_binding(&self, target: &Vector, range: f64) ->  Vec<ParticleRef<StaticParticle>> {
        // let result = Vec::<ParticleRef<StaticParticle>>::with_capacity(16);
        self.values()
            .filter(|ref_particle| {
                (ref_particle.particle.pos.x - target.x).abs() <= range 
                && (ref_particle.particle.pos.y - target.y).abs() <= range 
            })
            .collect()
    }
}