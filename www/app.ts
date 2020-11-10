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

import * as wasm from "../pkg/valo";
import * as PIXI from 'pixi.js';
import config from './config';
import { updateVisibleParticles } from './frame';

/// app class
class App {
    field: wasm.Field;
    pixi: PIXI.Application;
    movingParticlesContainer: PIXI.ParticleContainer;
    staticParticlesContainer: PIXI.ParticleContainer;
    fieldBorder: PIXI.Graphics;
    simulationTimeStart: number;

    constructor() {
        this.field = new wasm.Field(config.field.width, config.field.height);
        this.pixi = new PIXI.Application({
            backgroundColor: config.colours.background,
            width: config.display.width,
            height: config.display.height,
            antialias: true,
        });
        this.movingParticlesContainer = new PIXI.ParticleContainer(1000, { position: true, tint: true });
        this.staticParticlesContainer = new PIXI.ParticleContainer(2000, { position: true, tint: true });
        this.fieldBorder = new PIXI.Graphics();
        this.pixi.ticker.maxFPS = config.display.maxfps;

        this.simulationTimeStart = (new Date()).getTime();
    }

    /// load resources
    load() {
        app.pixi.loader.add([
            { name: 'particle', url: 'images/particle.png' }
        ]).load(() => this.setup());
    }

    /// field setup
    setup() {
        for (let i = 0; i < config.field.numParticles; i++) {
            this.field.add_particle()
        }

        this.pixi.stage.addChild(this.movingParticlesContainer)
        this.pixi.stage.addChild(this.staticParticlesContainer)

        // set particle displays to (0,0) in the center
        this.movingParticlesContainer.setTransform(
            config.display.width / 2, config.display.height / 2,
            config.display.width / (config.field.width * 2), config.display.height / (config.field.height * 2)
        )
        this.staticParticlesContainer.setTransform(
            config.display.width / 2, config.display.height / 2,
            config.display.width / (config.field.width * 2), config.display.height / (config.field.height * 2)
        )

        // some basic colours
        this.movingParticlesContainer.tint = config.colours.tintMoving;
        this.staticParticlesContainer.tint = config.colours.tintStatic;

        // field ui
        this.fieldBorder.lineStyle(4, config.colours.tintMoving, 1.0)
        this.fieldBorder.drawCircle(config.display.width / 2, config.display.height / 2, config.display.height / 2 - 4)
        app.pixi.stage.addChild(this.fieldBorder)

        let fieldMask = new PIXI.Graphics()
        fieldMask.lineStyle(0)
        fieldMask.beginFill(0xffffff)
        fieldMask.drawCircle(config.display.width / 2, config.display.height / 2, config.display.height / 2 - 4)
        fieldMask.endFill()
        this.pixi.stage.mask = fieldMask

        app.pixi.ticker.add(delta => this.loop(delta));
    }

    /// draw loop
    loop(delta: number) {
        updateVisibleParticles();

        for (let tick = 0; tick < config.field.ticksPerCall; tick++) {
            this.field.update_attachments();
            this.field.update_positions(0.7);
            this.field.update_velocities(0.7);

            if (this.field.num_moving_particles + this.field.num_static_particles < config.field.maxParticles) {
                // additional spawn rate from consumed particles
                const addSpawnRate = this.field.num_static_particles / (((new Date()).getTime() - this.simulationTimeStart) / 1000);
                // probability of spawn event happening in the last frame
                const expInterval = Math.exp(-(addSpawnRate + config.field.spawnRate) * this.pixi.ticker.elapsedMS / 1000);
                if (Math.random() > expInterval) {
                    this.field.add_boundary_particle(this.pixi.ticker.lastTime);
                }
            }
        }

        // draw ui
        this.fieldBorder.alpha =
            (config.field.maxParticles - this.field.num_static_particles - this.field.num_moving_particles) /
            config.field.maxParticles;

    }

    reset() {
        this.field = new wasm.Field(config.field.width, config.field.height);
    }
}

const app = new App;

export { app }