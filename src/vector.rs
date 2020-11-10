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

/// Vector
#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn normalize(v: &Vector) -> Vector {
        let len = 1. / (v.x * v.x + v.y * v.y).sqrt();
        Vector {
            x: v.x * len,
            y: v.y * len,
        }
    }

    pub fn distance_squared(one: &Vector, other: &Vector) -> f64 {
        let dx = other.x - one.x;
        let dy = other.y - one.y;
        dx * dx + dy * dy
    }

    pub fn dot(one: &Vector, other: &Vector) -> f64 {
        one.x * other.x + one.y * other.y
    }

    pub fn diff(to: &Vector, from: &Vector) -> Vector {
        Vector {
            x: to.x - from.x,
            y: to.y - from.y,
        }
    }
}
