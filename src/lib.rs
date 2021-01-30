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
mod container;
mod extfn;
mod field;
mod particle;
mod vector;

#[cfg(not(test))]
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    //console::log_1(&"Fair Random (tm) initialized".into());
    Ok(())
}

#[wasm_bindgen]
pub fn get_random() -> u32 {
    4u32
}

#[wasm_bindgen]
pub fn get_random_in_range(min: i32, max: i32) -> i32 {
    min + ((max - min) as f64 * extfn::random()).round() as i32
}
