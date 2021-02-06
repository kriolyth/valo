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

use wasm_bindgen::prelude::*;

use crate::extfn;

use crate::container::{MovingParticleContainer, Particle, StaticParticleContainer};
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
    Ok(Particle<StaticParticle>, BindingResult),
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
    // moving particles
    mp_container: MovingParticleContainer,
    // static particles
    sp_container: StaticParticleContainer,
    // binding configurations
    bind_cfgs: [BindingConfiguration; 1],
    // field dimensions, from -dim to +dim
    dimensions: Vector,
    // number of used particles
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
        let dimensions = Vector::new(half_width, half_height);
        Field {
            mp_container: MovingParticleContainer::new(MAX_MOVING, &dimensions),
            sp_container: StaticParticleContainer::new(MAX_STATIC, &dimensions),
            bind_cfgs: [BindingConfiguration::make_hexa()],
            dimensions,
        }
    }

    pub fn moving_particles_ptr(&self) -> *const MovingParticle {
        self.mp_container.as_ptr()
    }
    pub fn static_particles_ptr(&self) -> *const StaticParticle {
        self.sp_container.as_ptr()
    }
    pub fn moving_particles_count(&self) -> usize {
        self.mp_container.size()
    }
    pub fn static_particles_count(&self) -> usize {
        self.sp_container.size()
    }

    /// add a particle anywhere in the field
    pub fn add_particle(&mut self) {
        if !self.mp_container.is_full() {
            let pos = Field::random_pos_in_field(&self.dimensions);
            let vel = Field::random_vel_in_field();

            self.mp_container.add_particle(MovingParticle {
                pos,
                vel,
                since: 0.,
                flags: 0,
            });
        }
    }

    /// add a particle on the field boundary
    pub fn add_boundary_particle(&mut self, since: f64) {
        if !self.mp_container.is_full() {
            let pos = Field::random_boundary_pos_in_field(&self.dimensions);
            let vel = Field::random_vel_in_field();

            self.mp_container.add_particle(MovingParticle {
                pos,
                vel,
                since,
                flags: 0,
            });
        }
    }

    /// try adding a static particle directly (with respect to binding sites)
    pub fn add_static_particle(&mut self, pos: Vector) -> bool {
        if self.mp_container.is_full() || self.sp_container.is_full() {
            return false;
        }
        // to align added particles, we need to treat them as moving first and then convert them to static
        let vel = Field::random_vel_in_field();
        let new_particle = MovingParticle {
            pos,
            vel,
            since: 0.,
            flags: 0,
        };

        match self.check_single_particle_attachment(&new_particle) {
            AttachmentCheckResult::Ok(mut static_particle, binding) => {
                // apply binding
                self.convert_mp_to_static(&new_particle, &mut static_particle, binding)
            }
            AttachmentCheckResult::NoOtherParticle => {
                // no other static particles found in vicinity, just create a new one
                self.sp_container
                    .add_particle(StaticParticle {
                        pos,
                        rot: 0.,
                        binding_cfg_id: 0,
                    })
                    .is_some()
            }
            AttachmentCheckResult::SitesBusy => false,
        }
    }

    /// make a moving particle static by attaching to another static particle
    fn convert_mp_to_static(
        &mut self,
        moving_particle: &MovingParticle,
        static_particle: &mut Particle<StaticParticle>,
        binding_result: BindingResult,
    ) -> bool {
        !self.sp_container.is_full() &&
            // apply binding operation to convert moving particle to static
            if let Some(new_static_particle) = binding_result.apply_binding(
                moving_particle,
                &mut static_particle.particle,
                &self.bind_cfgs[0],
                &self.bind_cfgs[0],
            ) {
                // update bound static particle, because it is a copy of the real thing
                self.sp_container.update(&static_particle);
                // move to static list
                self.sp_container.add_particle(new_static_particle).is_some()
            }
            else { false }
    }

    /// update particle positions according to time delta
    pub fn update_positions(&mut self, delta: f64) {
        self.mp_container
            .apply(|p: &mut MovingParticle| p.pos += p.vel * delta);
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
        let field_dimenstions = self.dimensions;
        let num_static_particles = self.sp_container.size();

        self.mp_container.apply(|particle| {
            // particle can always change its direction unpredictably (Brownian motion)
            let new_dir = Field::random_vel_in_field();

            // define an attractor at the center, so that every particle is eventually caught
            let attractor_vector = Self::center_attractor_vector(&particle.pos, &field_dimenstions);
            // accelerate when approaching attractor
            let attractor_force = VELOCITY_FIELD_ATTENUATION
                * (0.2 / (0.2 + Vector::length(&attractor_vector).max(1.))
                    + 0.5 * num_static_particles as f64 / MAX_STATIC as f64);
            // Importantly, field force affects particle density, which determines growth features

            particle.vel = Vector::normalize(
                particle.vel
                    // velocity changes according to delta, but is always normalized afterwards
                    + Vector::normalize(attractor_vector * attractor_force + new_dir) * delta,
            );
        });
    }

    /// test particles that can attach and return a list of their indices
    /// (note the limits - at most 4 particles are returned per call)
    fn check_mp_attachment(
        &self,
    ) -> Vec<(
        Particle<MovingParticle>,
        Particle<StaticParticle>,
        BindingResult,
    )> {
        let mut results = Vec::<(
            Particle<MovingParticle>,
            Particle<StaticParticle>,
            BindingResult,
        )>::with_capacity(4);
        let bind_cfg = &self.bind_cfgs[0];

        'outer: for moving in self.mp_container.values() {
            'inner: for fixed in self.sp_container.select_for_binding(
                &moving.particle.pos,
                bind_cfg.radius()
            ) {
                if let Some(binding) =
                    BindingResult::get_binding(moving.particle, fixed.particle, bind_cfg, bind_cfg)
                {
                    results.push((moving.as_copy(), fixed.as_copy(), binding));
                    if results.len() == 4 {
                        break 'outer;
                    }
                    break 'inner;
                }
            }
        }
        results
    }

    /// test particles that can attach and return a list of their indices
    /// (note the limits - at most 4 particles are returned per call)
    fn check_single_particle_attachment(&self, moving: &MovingParticle) -> AttachmentCheckResult {
        let bind_cfg = &self.bind_cfgs[0];

        let closest_static_particles: Vec<_> = self
            .sp_container
            .select_for_binding(&moving.pos, bind_cfg.radius())
            .into_iter()
            .filter(|fixed| bind_cfg.close_enough_to_bind(&fixed.particle.pos, &moving.pos))
            .collect();
        if closest_static_particles.is_empty() {
            return AttachmentCheckResult::NoOtherParticle;
        }

        closest_static_particles
            .iter()
            .find_map(|fixed_ref| {
                BindingResult::get_binding(moving, fixed_ref.particle, bind_cfg, bind_cfg)
                    .map_or(None, |r: BindingResult| {
                        Some(AttachmentCheckResult::Ok(fixed_ref.as_copy(), r))
                    })
            })
            .unwrap_or(AttachmentCheckResult::SitesBusy)
    }

    /// update attachments and particles disposition
    pub fn update_attachments(&mut self) {
        let converted = self
            .check_mp_attachment()
            .into_iter()
            .filter_map(|(moving, mut fixed, bind_result)| {
                if self.convert_mp_to_static(&moving.particle, &mut fixed, bind_result) {
                    Some(moving.index)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        self.mp_container.remove_multiple_by_index(converted);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    impl std::fmt::Debug for Vector {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            fmt.debug_struct("Vector")
                .field("x", &self.x)
                .field("y", &self.y)
                .finish()
        }
    }

    impl std::fmt::Debug for StaticParticle {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                fmt,
                "StaticParticle {{ ports: {:#06b}, pos: {:?} }})",
                &(self.binding_cfg_id >> 32),
                &self.pos
            )
        }
    }

    #[wasm_bindgen_test]
    fn attachment() {
        let mut f = Field::new(200., 200.);
        f.bind_cfgs[0] = BindingConfiguration::make_square();
        assert!(f.add_static_particle(Vector::new(0., 0.)));

        let d = Vector { x: 1.0, y: 0.0 };
        f.mp_container.add_particle(MovingParticle {
            pos: Vector { x: 1.0, y: 0.0 },
            vel: d,
            since: 0.,
            flags: 0,
        });
        f.mp_container.add_particle(MovingParticle {
            pos: Vector { x: -5.0, y: 1.0 },
            vel: d,
            since: 0.,
            flags: 0,
        });
        f.mp_container.add_particle(MovingParticle {
            pos: Vector { x: 2.0, y: 2.0 },
            vel: d,
            since: 0.,
            flags: 0,
        });
        f.mp_container.add_particle(MovingParticle {
            pos: Vector { x: -5.0, y: 6.0 },
            vel: d,
            since: 0.,
            flags: 0,
        });

        let att = f.check_mp_attachment();
        assert_eq!(att[0].0.index, 0);
        assert_eq!(att[1].0.index, 2);
        assert_eq!(att.len(), 2);

        f.update_attachments();

        let att = f.check_mp_attachment();
        assert!(att.is_empty(), "Attachments found on second pass");
    }

    #[wasm_bindgen_test]
    fn attachment_multi() {
        let mut f = Field::new(200., 200.);
        f.bind_cfgs[0] = BindingConfiguration::make_square();
        assert!(f.add_static_particle(Vector::new(0., 0.)));

        let d = Vector { x: 1.0, y: 0.0 };
        f.sp_container.add_particle(StaticParticle {
            pos: Vector { x: 1.0, y: 0.0 },
            rot: 0.,
            binding_cfg_id: 0,
        });

        f.mp_container.add_particle(MovingParticle {
            pos: Vector { x: -5.0, y: 1.0 },
            vel: d,
            since: 0.,
            flags: 0,
        });
        f.mp_container.add_particle(MovingParticle {
            pos: Vector { x: 2.0, y: 2.0 },
            vel: d,
            since: 0.,
            flags: 0,
        });
        f.mp_container.add_particle(MovingParticle {
            pos: Vector { x: -5.0, y: 6.0 },
            vel: d,
            since: 0.,
            flags: 0,
        });

        let att = f.check_mp_attachment();
        assert_eq!(att.len(), 1);
        assert_eq!(att[0].0.index, 1);

        f.update_attachments();

        let att = f.check_mp_attachment();
        assert!(att.is_empty(), "Attachments found on second pass");
    }

    #[wasm_bindgen_test]
    fn custom_initial_static() {
        let mut f = Field::new(512., 512.);
        f.bind_cfgs[0] = BindingConfiguration::make_square();
        f.bind_cfgs[0].set_max_binds(2);
        f.bind_cfgs[0].set_radius(std::f64::consts::SQRT_2);
        // arrange 4 particles in a square, connected on sides
        assert!(f.add_static_particle(Vector::new(1., 0.)));
        assert_eq!(f.static_particles_count(), 1);
        assert!(f.add_static_particle(Vector::new(0., 1.)));
        assert_eq!(f.static_particles_count(), 2);
        assert!(f.add_static_particle(Vector::new(-1., 0.)));
        assert_eq!(f.static_particles_count(), 3);
        assert!(f.add_static_particle(Vector::new(0., -1.)));
        assert_eq!(f.static_particles_count(), 4);

        // Currently binds only occur on one particle (tree structure, not grid),
        // which leaves a way to add new particles even when there is seemingly no place for them.
        // So here for the particle to be rejected here we rely on ports busy, rather than binds
        assert!(
            !f.add_static_particle(Vector::new(-0.1, 0.1)),
            "{:?} {:?} {:?} {:?}",
            f.sp_container.at(0),
            f.sp_container.at(1),
            f.sp_container.at(2),
            f.sp_container.at(3)
        );
        assert_eq!(f.static_particles_count(), 4);
    }
}
