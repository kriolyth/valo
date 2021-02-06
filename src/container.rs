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

use crate::particle::{MovingParticle, Positionable, StaticParticle};
use crate::vector::Vector;
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

const BIN_DIMENSIONS: usize = 36;
/// Binning structure
struct Binnery {
    // bins of indices
    bins: Vec<Vec<usize>>,

    // offset and multiplier to convert arbitrary Vector to bin index
    offset: f64,
    multiplier: f64,
}

impl Binnery {
    /// Create an instance
    pub fn new(field_dimensions: &Vector) -> Self {
        Self {
            bins: vec![Vec::with_capacity(16); BIN_DIMENSIONS * BIN_DIMENSIONS],
            // field is from -x to +x, so we store shift and multiple offsets to convert them later
            // to indegral bins
            offset: field_dimensions.x.max(field_dimensions.y),
            multiplier: (BIN_DIMENSIONS as f64) / (2. * field_dimensions.x.max(field_dimensions.y)),
        }
    }

    pub fn to_linear(x: usize, y: usize) -> usize {
        x.min(BIN_DIMENSIONS - 1) + y.min(BIN_DIMENSIONS - 1) * BIN_DIMENSIONS
    }

    /// index into bin
    pub fn index(&self, pos: &Vector) -> usize {
        let f_bin: Vector = (Vector::new(self.offset, self.offset) + *pos) * self.multiplier;
        let x = (f_bin.x.max(0.).trunc() as usize).min(BIN_DIMENSIONS - 1);
        let y = (f_bin.y.max(0.).trunc() as usize).min(BIN_DIMENSIONS - 1);
        x + y * BIN_DIMENSIONS
    }

    pub fn add(&mut self, what: usize, pos: &Vector) {
        let bin = self.index(pos);
        self.bins[bin].push(what);
    }

    pub fn remove(&mut self, what: usize, pos: &Vector) {
        let bin = self.index(pos);
        self.bins[bin].retain(|&x| x != what);
    }

    pub fn update_index(&mut self, what: usize, new_value: usize, pos: &Vector) {
        let bin = self.index(pos);
        for value in self.bins[bin].iter_mut() {
            if *value == what {
                *value = new_value;
            }
        }
    }

    pub fn get_bin(&self, bin: usize) -> Vec<usize> {
        self.bins[bin].clone()
    }

    pub fn clear(&mut self) {
        self.bins.iter_mut().for_each(|bin| bin.clear());
    }
}

/// Container for particles
pub struct ParticleContainer<T>
where
    T: Copy + Positionable,
{
    particles: Vec<T>,
    bins: Binnery,
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
pub struct ContainerClusterIterator<'a, T> {
    particles: &'a Vec<T>,
    iter: slice::Iter<'a, Vec<usize>>,
}

impl<'a, T> Iterator for ContainerClusterIterator<'a, T> {
    type Item = Vec<ParticleRef<'a, T>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|index_vec| {
            index_vec
                .iter()
                .map(|&index| ParticleRef {
                    particle: &self.particles[index],
                    index,
                })
                .collect()
        })
    }
}

impl<T> ParticleContainer<T>
where
    T: Copy + Positionable,
{
    /// Create a new container with specified capacity
    pub fn new(capacity: usize, field_dimensions: &Vector) -> Self {
        Self {
            particles: Vec::with_capacity(capacity),
            bins: Binnery::new(field_dimensions),
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

    /// Provide an iterator of ParticleRef's over the container
    pub fn clusters(&self) -> ContainerClusterIterator<T> {
        ContainerClusterIterator {
            particles: &self.particles,
            iter: self.bins.bins.iter(),
        }
    }

    /// Apply an operator to particles
    pub fn apply<F>(&mut self, operator: F)
    where
        F: std::ops::Fn(&mut T) -> (),
    {
        self.bins.clear();
        self.particles.iter_mut().for_each(operator);
        for i in 0..self.particles.len() {
            let particle = &self.particles[i];
            self.bins.add(i, &particle.position());
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

    /// Add a new particle to the container
    pub fn add_particle(&mut self, particle: T) -> Option<()> {
        if self.is_full() {
            None
        } else {
            // add to own list
            self.particles.push(particle);
            // add index to binning structure
            self.bins
                .add(self.particles.len() - 1, &particle.position());
            Some(())
        }
    }

    /// Remove multiple particles using their indices
    pub fn remove_multiple_by_index(&mut self, mut items: Vec<usize>) {
        // We have to remove items in backwards order to successfully process
        // all of them.
        items.sort_unstable_by_key(|&k| std::cmp::Reverse(k));
        let own_size = self.particles.len();
        for index in items.iter().skip_while(|i| **i >= own_size) {
            // remove index from bin
            self.bins.remove(*index, &self.particles[*index].position());
            // last item is reordered, remove and insert it too
            let last_item_pos = self.particles.last().unwrap().position();
            // self.bins.remove(self.particles.len() - 1, &last_item_pos);
            // we can use swap_remove to avoid copying the vector tail over and over
            self.particles.swap_remove(*index);
            self.bins.update_index(self.particles.len(), *index, &last_item_pos);
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
    pub fn select_nearby_clusters(&self, target: &Vector, _range: f64) -> Vec<usize> {
        // choose most neightbouring bins; this puts restrictions on minimum bin size
        let this_index = self.bins.index(target);
        let this_y = this_index / BIN_DIMENSIONS;
        let this_x = this_index % BIN_DIMENSIONS;
        let mut bins = vec![
            Binnery::to_linear(this_x.saturating_sub(1), this_y.saturating_sub(1)),
            Binnery::to_linear(this_x, this_y.saturating_sub(1)),
            Binnery::to_linear(this_x + 1, this_y.saturating_sub(1)),
            Binnery::to_linear(this_x.saturating_sub(1), this_y),
            Binnery::to_linear(this_x, this_y),
            Binnery::to_linear(this_x + 1, this_y),
            Binnery::to_linear(this_x.saturating_sub(1), this_y + 1),
            Binnery::to_linear(this_x, this_y + 1),
            Binnery::to_linear(this_x + 1, this_y + 1),
        ];
        bins.sort_unstable();
        bins.dedup();
        bins
    }

    /// Select potential targets
    pub fn select_for_binding_from_clusters(
        &self,
        target: &Vector,
        range: f64,
        from_bins: &Vec<usize>,
    ) -> Vec<ParticleRef<StaticParticle>> {
        // convert indices from bins into particle refs and return final set of potential targets
        from_bins
            .iter()
            .fold(Vec::with_capacity(16), |mut accum, &bin| {
                self.bins.get_bin(bin).into_iter().for_each(|index| {
                    let particle = &self.particles[index];
                    if (particle.pos.x - target.x).abs() <= range
                        && (particle.pos.y - target.y).abs() <= range
                    {
                        accum.push(ParticleRef {
                            index,
                            particle: &self.particles[index],
                        })
                    }
                });
                accum
            })
    }
    /// Select potential targets
    pub fn select_for_binding(
        &self,
        target: &Vector,
        range: f64,
    ) -> Vec<ParticleRef<StaticParticle>> {
        // choose most neightbouring bins; this puts restrictions on minimum bin size
        let mut bins = vec![
            self.bins
                .index(&Vector::new(target.x - range, target.y - range)),
            self.bins
                .index(&Vector::new(target.x - range, target.y + range)),
            self.bins
                .index(&Vector::new(target.x + range, target.y - range)),
            self.bins
                .index(&Vector::new(target.x + range, target.y + range)),
        ];
        bins.sort_unstable();
        bins.dedup();

        // convert indices from bins into particle refs and return final set of potential targets
        bins.into_iter()
            .fold(Vec::with_capacity(16), |mut accum, bin| {
                self.bins.get_bin(bin).into_iter().for_each(|index| {
                    let particle = &self.particles[index];
                    if (particle.pos.x - target.x).abs() <= range
                        && (particle.pos.y - target.y).abs() <= range
                    {
                        accum.push(ParticleRef {
                            index,
                            particle: &self.particles[index],
                        })
                    }
                });
                accum
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn bin_index() {
        let binnery = Binnery::new(&Vector::new(10., 10.));
        assert_eq!(binnery.index(&Vector::new(-10., -10.)), 0, "Min field");
        assert_eq!(
            binnery.index(&Vector::new(10., 10.)),
            BIN_DIMENSIONS * BIN_DIMENSIONS - 1,
            "Max field"
        );
        assert_eq!(
            binnery.index(&Vector::new(-200., -200.)),
            0,
            "Out of bounds min field"
        );
        assert_eq!(
            binnery.index(&Vector::new(200., 200.)),
            BIN_DIMENSIONS * BIN_DIMENSIONS - 1,
            "Out of bounds max field"
        );
    }

    #[wasm_bindgen_test]
    fn choose_containers() {
        let st = StaticParticleContainer::new(10, &Vector::new(200., 200.));
        assert_eq!(
            st.select_nearby_clusters(&Vector::new(-205., -205.), 10.),
            vec![0, 1, BIN_DIMENSIONS, BIN_DIMENSIONS + 1],
            "Bins at start"
        );
        assert_eq!(
            st.select_nearby_clusters(&Vector::new(0., 0.), 10.).len(),
            9,
            "Center bins"
        );
        assert_eq!(
            st.select_nearby_clusters(&Vector::new(205., 205.), 10.),
            vec![
                BIN_DIMENSIONS * (BIN_DIMENSIONS - 1) - 2,
                BIN_DIMENSIONS * (BIN_DIMENSIONS - 1) - 1,
                BIN_DIMENSIONS * BIN_DIMENSIONS - 2,
                BIN_DIMENSIONS * BIN_DIMENSIONS - 1
            ],
            "Bins at end"
        );
    }
}
