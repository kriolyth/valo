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
use crate::vector::Vector;
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;

// valid ID configuration bits
const BIND_CFG_ID_MASK: u64 = 0xff;

/// A moving particle on the field
#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq)]
pub struct MovingParticle {
    pub pos: Vector,
    pub vel: Vector,
    pub since: f64, // engine time when this particle appeared
    pub flags: u64,
}

impl Default for MovingParticle {
    fn default() -> MovingParticle {
        MovingParticle {
            pos: Vector { x: 0., y: 0. },
            vel: Vector { x: 0., y: 0. },
            since: 0.,
            flags: 0,
        }
    }
}
#[wasm_bindgen]
impl MovingParticle {
    pub fn get_f64_size() -> usize {
        6
    }
}

/// A static particle on the field
#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq)]
pub struct StaticParticle {
    /// Static particle position
    pub pos: Vector,
    /// Static particle rotation relative to bind configuration (degrees)
    pub rot: f64,
    /// Binding configuration, packed
    /// [busy ports mask: u32, cfg id: u32]
    pub binding_cfg_id: u64,
}
impl Default for StaticParticle {
    fn default() -> StaticParticle {
        StaticParticle {
            pos: Vector { x: 0., y: 0. },
            rot: 0.,
            binding_cfg_id: 0,
        }
    }
}
impl StaticParticle {
    fn bind_config_and_port(cfg_id: u64, port: u8) -> u64 {
        (cfg_id & BIND_CFG_ID_MASK) | (1u64 << (32 + port))
    }
    fn is_port_free(&self, port: u8) -> bool {
        self.binding_cfg_id & (1u64 << (32 + port)) == 0
    }
    fn count_busy_ports(&self) -> u8 {
        (self.binding_cfg_id & (255u64 << 32)).count_ones() as u8
    }
    fn set_port_busy(&mut self, port: u8) {
        self.binding_cfg_id |= 1u64 << (32 + port);
    }
}

/// How to align a moving particle during attachment
#[derive(Copy, Clone, PartialEq)]
enum AttachmentAlignment {
    Zero,
    Port,
    Free,
}

/// Binding sites and their parameters
#[derive(Copy, Clone)]
pub struct BindingConfiguration {
    // sites as segments, degrees
    segments: [f64; 6],
    // attachment radius
    radius: f64,
    // limit number of binds to this particle (excluding its own attachment)
    max_binds: u8,
    // which segments can be used as binding for the moving particle
    // when it attaches to a static one (bitmask)
    attachment_site_mask: u8,
    // free rotation for attached particles (much less crystalline)
    align: AttachmentAlignment,
}

impl BindingConfiguration {
    pub const fn make_tri() -> Self {
        BindingConfiguration {
            segments: [120., 120., 120., 0., 0., 0.],
            radius: 5.,
            max_binds: 2,
            attachment_site_mask: 0b00_000111,
            align: AttachmentAlignment::Zero,
        }
    }

    pub const fn make_square() -> Self {
        BindingConfiguration {
            segments: [90., 90., 90., 90., 0., 0.],
            radius: 5.,
            max_binds: 3,
            attachment_site_mask: 0b00_001111,
            align: AttachmentAlignment::Zero,
        }
    }

    pub const fn make_penta() -> Self {
        BindingConfiguration {
            segments: [72., 72., 72., 72., 72., 0.],
            radius: 5.,
            max_binds: 4,
            attachment_site_mask: 0b00_011111,
            align: AttachmentAlignment::Zero,
        }
    }

    pub const fn make_hexa() -> Self {
        BindingConfiguration {
            segments: [60., 60., 60., 60., 60., 60.],
            radius: 5.,
            max_binds: 5,
            attachment_site_mask: 0b00_111111,
            align: AttachmentAlignment::Zero,
        }
    }

    pub fn set_max_binds(&mut self, max_binds: u8) {
        self.max_binds = max_binds;
    }
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }

    fn angle_to_port(&self, angle: f64) -> Option<usize> {
        let mut start = 0f64;
        let circle_angle = angle.rem_euclid(360.);
        for (index, &width) in self.segments.iter().enumerate() {
            if start <= circle_angle && circle_angle < start + width {
                return Some(index);
            }
            start += width;
        }
        None
    }

    fn port_to_angle(&self, port: usize) -> Option<f64> {
        self.segments
            .get(port)
            .filter(|&&w| w > 0.)
            .map(|width| self.segments.iter().take(port).sum::<f64>() + width / 2.)
    }

    fn is_port_attachable(&self, port: usize) -> bool {
        self.attachment_site_mask & (1 << port) != 0
    }

    pub fn close_enough_to_bind(&self, pos1: &Vector, pos2: &Vector) -> bool {
        Vector::distance_squared(pos1, pos2) <= (self.radius * self.radius * 1.0000001)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct BindingResult {
    rot: f64, // rotation applied to moving particle to align with own binding site center
    site_at_static: u8, // site at static particle
    site_at_moving: u8, // site at moving particle (rotated)
}
impl BindingResult {
    /// Find closest port at static particle that a moving particle can attach to,
    /// and match to a port at moving particle, according to relative particles configuration
    /// Return BindingResult, if a match is possible, None if not
    pub fn get_binding(
        mp: &MovingParticle,
        sp: &StaticParticle,
        mp_bind_cfg: &BindingConfiguration,
        sp_bind_cfg: &BindingConfiguration,
    ) -> Option<BindingResult> {
        if !sp_bind_cfg.close_enough_to_bind(&mp.pos, &sp.pos) || sp.count_busy_ports() >= sp_bind_cfg.max_binds {
            return None;
        }
        let diff_to_mp = Vector::diff(&mp.pos, &sp.pos);
        let angle_to_mp = extfn::atan2(diff_to_mp.y, diff_to_mp.x) / PI * 180.;
        sp_bind_cfg
            .angle_to_port(angle_to_mp - sp.rot)
            .and_then(|port_at_sp| {
                // we have a match
                let is_bind_allowed: bool =
                    // is port open in bind configuration
                    (1 << port_at_sp) & sp_bind_cfg.attachment_site_mask != 0 &&
                    // is it not busy with other particle
                    (1 << port_at_sp) & (sp.binding_cfg_id >> 32) == 0
                ;
                if !is_bind_allowed {
                    return None;
                }

                // alignment applied to rotation angle:
                // - free makes non-crystalline free growing attachments
                // - port makes irregular crystals grow
                // - zero removes variance
                match sp_bind_cfg.align {
                    AttachmentAlignment::Free => {
                        // use movement direction as rotation
                        let rot = mp.vel.y.atan2(mp.vel.x) / PI * 180.;
                        mp_bind_cfg
                            .angle_to_port(180. + angle_to_mp - rot)
                            .filter(|&port_at_mp| mp_bind_cfg.is_port_attachable(port_at_mp))
                            .map(|port_at_mp| BindingResult {
                                rot,
                                site_at_static: port_at_sp as u8,
                                site_at_moving: port_at_mp as u8,
                            })
                    }
                    AttachmentAlignment::Port => {
                        // align port centers of moving and static particles
                        // (particle is expected to be pulled up to the rendezvous point)
                        let rot = mp.vel.y.atan2(mp.vel.x) / PI * 180.;
                        mp_bind_cfg
                            .angle_to_port(180. + angle_to_mp - rot)
                            .filter(|&port_at_mp| mp_bind_cfg.is_port_attachable(port_at_mp))
                            .map(|port_at_mp| BindingResult {
                                rot: 0., /* ignore until binding */
                                site_at_static: port_at_sp as u8,
                                site_at_moving: port_at_mp as u8,
                            })
                    }
                    AttachmentAlignment::Zero => {
                        // particle movement direction is irrelevant
                        // (particle is considered to be pulled up to the rendezvous point)
                        mp_bind_cfg
                            .angle_to_port(180. + angle_to_mp - sp.rot)
                            .filter(|&port_at_mp| mp_bind_cfg.is_port_attachable(port_at_mp))
                            .map(|port_at_mp| BindingResult {
                                rot: sp.rot,
                                site_at_static: port_at_sp as u8,
                                site_at_moving: port_at_mp as u8,
                            })
                    }
                }
            })
    }

    /// Apply a BindingResult to a pair of moving/static particles
    /// Return a static particle that is created as a result of binidng
    pub fn apply_binding(
        self,
        mp: &MovingParticle,
        sp: &mut StaticParticle,
        mp_bind_cfg: &BindingConfiguration,
        sp_bind_cfg: &BindingConfiguration,
    ) -> Option<StaticParticle> {
        // safety check, silently fail if a particle cannot be attached
        if !sp.is_port_free(self.site_at_static) || sp.count_busy_ports() >= sp_bind_cfg.max_binds {
            return None;
        }
        sp.set_port_busy(self.site_at_static);

        // get angle at binding site
        let angle = sp_bind_cfg
            .port_to_angle(self.site_at_static as usize)
            .map(|x| x + sp.rot)
            .unwrap_or(0.);
        let pos = match sp_bind_cfg.align {
            AttachmentAlignment::Free => mp.pos,
            _ => {
                let sincos_angle = (angle * PI / 180.).sin_cos();
                Vector::diff(
                    &sp.pos,
                    &Vector {
                        x: -sincos_angle.1 * sp_bind_cfg.radius,
                        y: -sincos_angle.0 * sp_bind_cfg.radius,
                    },
                )
            }
        };

        Some(StaticParticle {
            pos,
            rot: match sp_bind_cfg.align {
                AttachmentAlignment::Port => {
                    // set rotation of the new particle so that it aligns its port
                    mp_bind_cfg
                        .port_to_angle(self.site_at_moving as usize)
                        .map_or(0., |mp_angle| (180. + angle - mp_angle))
                }
                _ => self.rot,
            },
            binding_cfg_id: StaticParticle::bind_config_and_port(0, self.site_at_moving),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    impl std::fmt::Debug for BindingResult {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            fmt.debug_struct("BindingResult")
                .field("site_at_static", &self.site_at_static)
                .field("site_at_moving", &self.site_at_moving)
                .field("rotation", &self.rot)
                .finish()
        }
    }

    #[wasm_bindgen_test]
    fn bind_align() {
        /*
            Port alignment

                                  ▲
                    v      .--  ◄←p ►
                  >(o)< --'       ▼
                    ^
                Receiving port 0, moving port 0
                "p" is shifted down to center at port

            Receiving port and moving port are determined according to relative
            position and orientation. Moving particle rotation is obtained from
            velocity vector.
            Moving particle is snapped to align to the center of receiving port
            and rotated to center at its own port


            Zero alignment

                                  ▲
                    v      .--  ◄ p ►
                  >(o)< --'       ▼
                    ^
                Receiving port 0, moving port 2
                "p" is shifted down to center at port

            Receiving port and moving port are determined according to relative
            position. Moving particle rotation/direction is irrelevant.
            Moving particle is snapped to align to the center of receiving port.
            Resulting particles rotation is set to receving particle.
        */
        let bind_port = BindingConfiguration {
            segments: [90., 90., 90., 90., 0., 0.],
            radius: 5.,
            max_binds: 3,
            attachment_site_mask: 0b00_001111,
            align: AttachmentAlignment::Port,
        };
        let bind_zero = BindingConfiguration {
            segments: [90., 90., 90., 90., 0., 0.],
            radius: 5.,
            max_binds: 3,
            attachment_site_mask: 0b00_001111,
            align: AttachmentAlignment::Zero,
        };

        let mut sp = StaticParticle {
            pos: Vector { x: 0., y: 0. },
            rot: 0.,
            binding_cfg_id: 0,
        };
        let mut mp = MovingParticle {
            pos: Vector { x: 1., y: 1. },
            vel: Vector { x: 1., y: 0. },
            since: 0.,
            flags: 0,
        };
        // sp and mp rotation
        for ro in 0..4 {
            sp.rot = 90. * (ro as f64);
            let angle = ro as f64 * PI / 2.;
            mp.vel = Vector {
                x: angle.cos(),
                y: angle.sin(),
            };
            let bind_result = BindingResult::get_binding(&mp, &sp, &bind_port, &bind_port);
            assert_eq!(
                bind_result,
                Some(BindingResult {
                    rot: 0.,
                    site_at_static: (4 - ro) % 4,
                    site_at_moving: (6 - ro) % 4
                }),
                "Zero alignment bind failed at ro {}",
                ro
            );
            let particle =
                bind_result
                    .unwrap()
                    .apply_binding(&mp, &mut sp.clone(), &bind_port, &bind_port);
            assert_eq!(
                particle.unwrap().rot.rem_euclid(360.),
                90. * (ro as f64),
                "Wrong binding angle at ro {}",
                ro
            );
            let bind_result = BindingResult::get_binding(&mp, &sp, &bind_zero, &bind_zero);
            assert_eq!(
                bind_result,
                Some(BindingResult {
                    rot: sp.rot,
                    site_at_static: (4 - ro) % 4,
                    site_at_moving: (6 - ro) % 4
                }),
                "Zero alignment bind failed at ro {}",
                ro
            );
        }
    }

    #[wasm_bindgen_test]
    fn port_angle() {
        let cfg = BindingConfiguration::make_square();
        assert_eq!(cfg.angle_to_port(0.), Some(0));
        assert_eq!(cfg.angle_to_port(30.), Some(0));
        assert_eq!(cfg.angle_to_port(90.), Some(1));
        assert_eq!(cfg.angle_to_port(145.), Some(1));
        assert_eq!(cfg.angle_to_port(175.), Some(1));
        assert_eq!(cfg.angle_to_port(180.), Some(2));
        assert_eq!(cfg.angle_to_port(270.), Some(3));
        assert_eq!(cfg.angle_to_port(355.), Some(3));
        assert_eq!(cfg.angle_to_port(365.), Some(0));
        assert_eq!(cfg.angle_to_port(-20.), Some(3));

        assert_eq!(cfg.port_to_angle(0), Some(45.));
        assert_eq!(cfg.port_to_angle(1), Some(135.));
        assert_eq!(cfg.port_to_angle(2), Some(225.));
        assert_eq!(cfg.port_to_angle(3), Some(315.));
        assert_eq!(cfg.port_to_angle(4), None);
        assert_eq!(cfg.port_to_angle(12), None);
    }
}
