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

import { MovingParticle } from '../pkg/valo';
import { memory } from '../pkg/valo_bg.wasm';
import * as PIXI from 'pixi.js';

import { app } from './app';

function addSprites(spriteContainer: PIXI.Container, numSprites: number) {
    for (; numSprites-- > 0;) {
        let pixie = new PIXI.Sprite(app.pixi.loader.resources['particle'].texture)
        spriteContainer.addChild(pixie)
    }
}

function updateVisibleParticles() {
    regrowParticleContainer(app.movingParticlesContainer, app.field.num_moving_particles)
    regrowParticleContainer(app.staticParticlesContainer, app.field.num_static_particles)

    // update moving particles positions
    const particle_size = MovingParticle.get_f64_size();
    const movingParticlesView = new Float64Array(
        memory.buffer,
        app.field.moving_particles_ptr(),
        app.field.num_moving_particles * particle_size);
    const now = app.pixi.ticker.lastTime;
    const alpha_offset = app.fieldBorder.alpha; // match border

    for (let i = 0; i < app.field.num_moving_particles; i++) {
        app.movingParticlesContainer.children[i].position.set(
            movingParticlesView[i * particle_size], movingParticlesView[i * particle_size + 1])
        // moving particles have a timestamp when they appeared; we use this timestamp
        // to have particles gradually achieve full glow, 
        // and offset initial glow to match border 
        app.movingParticlesContainer.children[i].alpha = Math.min(
            1.0, 
            alpha_offset + (now - movingParticlesView[i * particle_size + 4]) / (3000));
    }

    // update static particles positions
    const staticParticlesView = new Float64Array(
        memory.buffer,
        app.field.static_particles_ptr(),
        app.field.num_static_particles * 4);
    for (let i = 0; i < app.field.num_static_particles; i++) {
        app.staticParticlesContainer.children[i].position.set(
            staticParticlesView[i * 4], staticParticlesView[i * 4 + 1])
    }
}

function regrowParticleContainer(spriteContainer: PIXI.Container, actualCount: number) {
    if (spriteContainer.children.length < actualCount) {
        // add more sprites
        addSprites(spriteContainer, actualCount - spriteContainer.children.length)
    } else if (spriteContainer.children.length > actualCount) {
        // remove sprites
        spriteContainer.removeChildren(actualCount)
    }
}

export {
    updateVisibleParticles
}