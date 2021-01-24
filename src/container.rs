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

use crate::particle::MovingParticle;
use std::slice;

const MAX_MOVING: usize = 1000;

pub struct ParticleRef<'a, T>{
    pub particle: &'a T,
    pub index: usize,
}
pub struct Particle<T> where T: Copy {
    pub particle: T,
    pub index: usize,
}
impl<'a, T> ParticleRef<'a, T> where T: Copy{
    pub fn as_copy(&self) -> Particle<T> {
        Particle::<T> {
            particle: *self.particle,
            index: self.index
        }
    }
}

pub struct MovingParticleContainer {
    particles: [MovingParticle; MAX_MOVING],
    num_particles: usize,
}

pub struct ContainerIterator<'a, T> {
    iter: std::iter::Take<std::iter::Enumerate<slice::Iter<'a, T>>>,
}

impl<'a, T> Iterator for ContainerIterator<'a, T> {
    type Item = ParticleRef<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, particle)| ParticleRef { particle, index })
    }
}

impl MovingParticleContainer {
    pub fn new() -> Self {
        MovingParticleContainer {
            particles: [MovingParticle::default(); MAX_MOVING],
            num_particles: 0,
        }
    }

    pub fn is_full(&self) -> bool {
        self.num_particles >= MAX_MOVING
    }
    pub fn size(&self) -> usize {
        self.num_particles
    }

    pub fn add_particle(&mut self, particle: MovingParticle) -> Option<()> {
        if self.is_full() {
            None
        } else {
            self.particles[self.num_particles] = particle;
            self.num_particles += 1;
            Some(())
        }
    }

    pub fn values(&self) -> ContainerIterator<MovingParticle> {
        ContainerIterator {
            iter: self.particles.iter().enumerate().take(self.num_particles),
        }
    }
    pub fn apply<F>(&mut self, operator: F)
    where
        F: std::ops::Fn(&mut MovingParticle) -> (),
    {
        self.particles.iter_mut().for_each(operator);
    }

    pub fn remove_one(&mut self, ptr: ParticleRef<MovingParticle>) {
        if self.particles[ptr.index] == *ptr.particle {
            self.particles[ptr.index] = self.particles[self.num_particles - 1];
            self.num_particles -= 1;
        }
    }
    pub fn remove_one_by_index(&mut self, index: usize) {
        if index < self.num_particles {
            self.particles[index] = self.particles[self.num_particles - 1];
            self.num_particles -= 1;
        }
    }
    pub fn remove_multiple_by_index(&mut self, mut items: Vec<usize>) {
        items.sort_unstable_by_key(|&k| std::cmp::Reverse(k));
        for index in items.iter_mut() {
            self.remove_one_by_index(*index);
        }
    }
    pub fn as_ptr(&self) -> *const MovingParticle {
        self.particles.as_ptr()
    }
}
