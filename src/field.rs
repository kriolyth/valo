/*
   Copyright 2020 Alexander Efremkin

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

use crate::extfn;
use wasm_bindgen::prelude::*;

use crate::particle::{BindingConfiguration, BindingResult, MovingParticle, StaticParticle};
use crate::vector::*;

const MAX_MOVING: usize = 1000;
const MAX_STATIC: usize = 5000;

/// Particles are affected by field force; e.g. centripetal force
/// to prevent them from scattering away.
/// Field force, different at every point, is multiplied by this constant
const VELOCITY_FIELD_ATTENUATION: f64 = 1.0;

/// Result for attachment possibility check
/// To implement a custom starting static particle placement, we should be able
/// to arbitrarily position new static particles. However, to maintain a grid,
/// we must also go over standard attachment procdure in case other particles are
/// already there. This means that "no particles to attach to" and
/// "there are particles, but they are all busy" are different answers to the question
/// of whether a new static particle can be added at a specified location
enum AttachmentCheckResult {
    Ok(usize, BindingResult),
    SitesBusy,
    NoOtherParticle,
}

/// Renderable field
/// TODO:
/// 1. Multiple particle types (per bind point configuration)
/// 2. (optimization) Have an "accepting" index into static particles
///    that have not exceeded their bind point limit
/// 3. Global "currents" grid
/// 4. (optimization) collision bins?
#[wasm_bindgen]
pub struct Field {
    // particles (with hard limit)
    moving_particles: [MovingParticle; MAX_MOVING],
    // particles (with hard limit)
    static_particles: [StaticParticle; MAX_STATIC],
    // binding configurations
    bind_cfgs: [BindingConfiguration; 1],
    // field dimensions, from -dim to +dim
    dimensions: Vector,
    // number of used particles
    pub num_moving_particles: usize,
    pub num_static_particles: usize,
}

#[wasm_bindgen]
impl Field {
    /// generate random position in the field (elliptical field)
    fn random_pos_in_field(dimensions: &Vector) -> Vector {
        loop {
            let v = Vector {
                x: (extfn::random() * 2. - 1.) * dimensions.x,
                y: (extfn::random() * 2. - 1.) * dimensions.y,
            };
            if v.x * v.x / (dimensions.x * dimensions.x) + v.y * v.y / (dimensions.y * dimensions.y)
                <= 1.
            {
                return v;
            }
        }
    }
    /// generate random position on the field boundary (elliptical field)
    fn random_boundary_pos_in_field(dimensions: &Vector) -> Vector {
        // this is slightly wrong, because densities will skew in a truly elliptical field
        let mut v = Field::random_vel_in_field();
        v.x *= dimensions.x;
        v.y *= dimensions.y;
        v
    }

    /// generate random velocity vector with length 1
    fn random_vel_in_field() -> Vector {
        let theta_sc = (extfn::random() * 6.28).sin_cos();
        Vector {
            x: theta_sc.0,
            y: theta_sc.1,
        }
    }

    #[wasm_bindgen(constructor)]
    pub fn new(half_width: f64, half_height: f64) -> Field {
        Field {
            moving_particles: [MovingParticle::default(); MAX_MOVING],
            static_particles: [StaticParticle::default(); MAX_STATIC],
            bind_cfgs: [BindingConfiguration::make_hexa()],
            dimensions: Vector {
                x: half_width,
                y: half_height,
            },
            num_moving_particles: 0,
            num_static_particles: 0,
        }
    }

    pub fn moving_particles_ptr(&self) -> *const MovingParticle {
        self.moving_particles.as_ptr()
    }
    pub fn static_particles_ptr(&self) -> *const StaticParticle {
        self.static_particles.as_ptr()
    }

    /// add a particle anywhere in the field
    pub fn add_particle(&mut self) {
        if self.num_moving_particles < MAX_MOVING - 1 {
            let pos = Field::random_pos_in_field(&self.dimensions);
            let vel = Field::random_vel_in_field();
            self.moving_particles[self.num_moving_particles] = MovingParticle {
                pos,
                vel,
                since: 0.,
                flags: 0,
            };
            self.num_moving_particles += 1
        }
    }

    /// add a particle on the field boundary
    pub fn add_boundary_particle(&mut self, since: f64) {
        if self.num_moving_particles < MAX_MOVING - 1 {
            let pos = Field::random_boundary_pos_in_field(&self.dimensions);
            let vel = Field::random_vel_in_field();
            self.moving_particles[self.num_moving_particles] = MovingParticle {
                pos,
                vel,
                since,
                flags: 0,
            };
            self.num_moving_particles += 1
        }
    }

    /// try adding a static particle directly (with respect to binding sites)
    pub fn add_static_particle(&mut self, pos: Vector) -> bool {
        if self.num_moving_particles >= MAX_MOVING || self.num_static_particles >= MAX_STATIC {
            return false;
        }
        let vel = Field::random_vel_in_field();
        self.moving_particles[self.num_moving_particles] = MovingParticle {
            pos,
            vel,
            since: 0.,
            flags: 0,
        };
        self.num_moving_particles += 1;
        // self.convert_particle_to_static(moving_particle_idx: usize, static_particle_idx: usize, binding_result: BindingResult)
        match self.check_single_particle_attachment(self.num_moving_particles - 1) {
            AttachmentCheckResult::Ok(index_static, binding) => {
                // apply binding
                self.convert_particle_to_static(
                    self.num_moving_particles - 1,
                    index_static,
                    binding,
                );
                true
            }
            AttachmentCheckResult::NoOtherParticle => {
                // no other static particles found in vicinity, just create a new one
                self.static_particles[self.num_static_particles] = StaticParticle {
                    pos,
                    rot: 0.,
                    binding_cfg_id: 0,
                };
                self.num_static_particles += 1;
                self.num_moving_particles -= 1;
                true
            }
            AttachmentCheckResult::SitesBusy => false,
        }
    }

    /// make a moving particle static
    fn convert_particle_to_static(
        &mut self,
        moving_particle_idx: usize,
        static_particle_idx: usize,
        binding_result: BindingResult,
    ) {
        if moving_particle_idx < self.num_moving_particles
            && self.num_static_particles < MAX_STATIC - 1
        {
            // apply binding operation to convert moving particle to static
            if let Some(new_static_particle) = binding_result.apply_binding(
                &self.moving_particles[moving_particle_idx],
                &mut self.static_particles[static_particle_idx],
                &self.bind_cfgs[0],
                &self.bind_cfgs[0],
            ) {
                // move to static list
                self.static_particles[self.num_static_particles] = new_static_particle;
                self.num_static_particles += 1;
                // replace moved particle with the last one from the list
                self.moving_particles[moving_particle_idx] =
                    self.moving_particles[self.num_moving_particles - 1];
                self.num_moving_particles -= 1;
            }
        }
    }

    /// update particle positions according to time delta
    pub fn update_positions(&mut self, delta: f64) {
        for particle in &mut self.moving_particles {
            particle.pos += particle.vel * delta;
        }
    }

    /// simple center attractor vector, diminishes at the center
    fn center_attractor_vector(pos: &Vector, field_dim: &Vector) -> Vector {
        Vector {
            x: -(pos.x / field_dim.x),
            y: -(pos.y / field_dim.y),
        }
    }

    /// update particle velocities according to time delta
    pub fn update_velocities(&mut self, delta: f64) {
        for particle in &mut self.moving_particles {
            // particle can always change its direction unpredictably (Brownian motion)
            let new_dir = Field::random_vel_in_field();

            // define an attractor at the center, so that every particle is eventually caught
            let attractor_vector = Self::center_attractor_vector(&particle.pos, &self.dimensions);
            // accelerate when approaching attractor
            let attractor_force = VELOCITY_FIELD_ATTENUATION
                * (0.2 / (0.2 + Vector::length(&attractor_vector).max(1.))
                    + 0.5 * self.num_static_particles as f64 / MAX_STATIC as f64);
            // Importantly, field force affects particle density, which determines growth features

            particle.vel = Vector::normalize(
                particle.vel
                    // velocity changes according to delta, but is always normalized afterwards
                    + Vector::normalize(attractor_vector * attractor_force + new_dir) * delta,
            );
        }
    }

    /// test particles that can attach and return a list of their indices
    /// (note the limits - at most 4 particles are returned per call)
    fn check_attachment(&self) -> [(usize, usize, Option<BindingResult>); 4] {
        let mut result = [(MAX_MOVING, MAX_STATIC, None); 4];
        let mut n_results = 0;
        let bind_cfg = &self.bind_cfgs[0];

        'outer: for (index, moving) in self
            .moving_particles
            .iter()
            .enumerate()
            .take(self.num_moving_particles)
        {
            'inner: for (index_static, fixed) in self
                .static_particles
                .iter()
                .enumerate()
                .take(self.num_static_particles)
            {
                if let Some(binding) = BindingResult::get_binding(&moving, fixed, bind_cfg, bind_cfg)
                {
                    result[n_results] = (index, index_static, Some(binding));
                    n_results += 1;
                    if n_results == 4 {
                        break 'outer;
                    }
                    break 'inner;
                }
            }
        }
        result
    }

    /// test particles that can attach and return a list of their indices
    /// (note the limits - at most 4 particles are returned per call)
    fn check_single_particle_attachment(&self, index_moving: usize) -> AttachmentCheckResult {
        let bind_cfg = &self.bind_cfgs[0];

        let moving = &self.moving_particles[index_moving];
        let closest_static_particles: Vec<(usize, &StaticParticle)> = self
            .static_particles
            .iter()
            .enumerate()
            .take(self.num_static_particles)
            .filter(|(_index_static, fixed)| bind_cfg.close_enough_to_bind(&fixed.pos, &moving.pos))
            .collect();
        if closest_static_particles.is_empty() {
            return AttachmentCheckResult::NoOtherParticle;
        }

        closest_static_particles
            .iter()
            .find_map(|(index_static, fixed)| {
                BindingResult::get_binding(moving, fixed, bind_cfg, bind_cfg)
                    .map_or(None, |r: BindingResult| {
                        Some(AttachmentCheckResult::Ok(*index_static, r))
                    })
            })
            .unwrap_or(AttachmentCheckResult::SitesBusy)
    }

    /// update attachments and particles disposition
    pub fn update_attachments(&mut self) {
        let mut attachments: [(usize, usize, Option<BindingResult>); 4] = self.check_attachment();
        // sort descending, because particle conversion changes indices of
        // processed vertices in a way that would interfere with attachment results
        attachments.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        for &(index_moving, index_static, bind_result_opt) in attachments.iter() {
            if let Some(bind_result) = bind_result_opt {
                self.convert_particle_to_static(index_moving, index_static, bind_result);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn attachment() {
        let mut f = Field::new(10., 10.);
        assert!(f.add_static_particle(Vector::new(0., 0.)));
        let d = Vector { x: 1.0, y: 0.0 };
        f.moving_particles[0] = MovingParticle {
            pos: Vector { x: 1.0, y: 0.0 },
            vel: d,
            since: 0.,
            flags: 0,
        };
        f.moving_particles[1] = MovingParticle {
            pos: Vector { x: -5.0, y: 1.0 },
            vel: d,
            since: 0.,
            flags: 0,
        };
        f.moving_particles[2] = MovingParticle {
            pos: Vector { x: 2.0, y: 2.0 },
            vel: d,
            since: 0.,
            flags: 0,
        };
        f.moving_particles[3] = MovingParticle {
            pos: Vector { x: -5.0, y: 6.0 },
            vel: d,
            since: 0.,
            flags: 0,
        };
        f.num_moving_particles = 4;

        let att = f.check_attachment();
        assert_eq!(att[0].0, 0);
        assert_eq!(att[1].0, 2);
        assert_eq!(att[2].0, MAX_MOVING);
        assert_eq!(att[3].0, MAX_MOVING);

        f.update_attachments();

        let att = f.check_attachment();
        assert_eq!(att[0].0, MAX_MOVING);
        assert_eq!(att[1].0, MAX_MOVING);
        assert_eq!(att[2].0, MAX_MOVING);
        assert_eq!(att[3].0, MAX_MOVING);
    }

    #[wasm_bindgen_test]
    fn attachment_multi() {
        let mut f = Field::new(10., 10.);
        assert!(f.add_static_particle(Vector::new(0., 0.)));
        let d = Vector { x: 1.0, y: 0.0 };
        f.static_particles[1] = StaticParticle {
            pos: Vector { x: 1.0, y: 0.0 },
            rot: 0.,
            binding_cfg_id: 0,
        };
        f.num_static_particles = 2;

        f.moving_particles[0] = MovingParticle {
            pos: Vector { x: -5.0, y: 1.0 },
            vel: d,
            since: 0.,
            flags: 0,
        };
        f.moving_particles[1] = MovingParticle {
            pos: Vector { x: 2.0, y: 2.0 },
            vel: d,
            since: 0.,
            flags: 0,
        };
        f.moving_particles[2] = MovingParticle {
            pos: Vector { x: -5.0, y: 6.0 },
            vel: d,
            since: 0.,
            flags: 0,
        };
        f.num_moving_particles = 3;

        let att = f.check_attachment();
        assert_eq!(att[0].0, 1);
        assert_eq!(att[1].0, MAX_MOVING);

        f.update_attachments();

        let att = f.check_attachment();
        assert_eq!(att[0].0, MAX_MOVING);
    }
}
