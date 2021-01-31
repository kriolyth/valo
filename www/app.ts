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

    paused: boolean;
    ready: boolean;

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
        this.ready = false;
        this.paused = false;
    }

    /// load resources
    load() {
        this.pixi.loader.add([
            { name: 'particle', url: 'images/particle.png' }
        ]).load(() => { this.setup() });
    }

    /// field setup
    setup() {
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
        this.pixi.stage.addChild(this.fieldBorder)

        let fieldMask = new PIXI.Graphics()
        fieldMask.lineStyle(0)
        fieldMask.beginFill(0xffffff)
        fieldMask.drawCircle(config.display.width / 2, config.display.height / 2, config.display.height / 2 - 4)
        fieldMask.endFill()
        this.pixi.stage.mask = fieldMask

        this.pixi.ticker.add(delta => this.loop(delta));

        this.start();
    }

    /// draw loop
    loop(delta: number) {

        updateVisibleParticles();
        
        if (this.ready && !this.paused) {
            for (let tick = 0; tick < config.field.ticksPerCall; tick++) {
                this.field.update_attachments();

                // Moving particles have velocities; this is how fast their positions changes in velocity direction
                this.field.update_positions(0.75);

                // Velocities are also updated every time; this value is how fast velocity changes due to environment effects
                this.field.update_velocities(0.8);

                if (this.field.moving_particles_count() + this.field.static_particles_count() < config.field.maxParticles) {
                    // additional spawn rate from consumed particles
                    const addSpawnRate = this.field.static_particles_count() / (((new Date()).getTime() - this.simulationTimeStart) / 1000);
                    // probability of spawn event happening in the last frame
                    const expInterval = Math.exp(-(addSpawnRate + config.field.spawnRate) * this.pixi.ticker.elapsedMS / 1000);
                    if (Math.random() > expInterval) {
                        this.field.add_boundary_particle(this.pixi.ticker.lastTime);
                    }
                }
            }
        }

        // draw ui
        this.fieldBorder.alpha =
            (config.field.maxParticles - this.field.static_particles_count() - this.field.moving_particles_count()) /
            config.field.maxParticles;

    }

    /// Reset the simulation
    reset() {
        this.field = new wasm.Field(config.field.width, config.field.height);
        this.ready = false;
    }

    /// Start a new simulation
    start() {

        // add a bunch of movers
        for (let i = 0; i < config.field.startParticles; i++) {
            this.field.add_particle()
        }

        // add a center particle
        if (this.field.static_particles_count() == 0)
            this.field.add_static_particle(new wasm.Vector(0., 0.))

        this.simulationTimeStart = (new Date()).getTime();
        this.ready = true;
        this.resume();
    }

    /// Pause a currently active simulation
    pause() {
        // this.pixi.stop()
        this.paused = true
    }
    /// Resume a currently active simulation
    resume() {
        this.paused = false
        // this.pixi.start()
    }
    isPaused() {
        return this.paused
    }
    isReady() {
        return this.ready
    }

    addCustomParticle(viewX: number, viewY: number) {
        const px = ((viewX / config.display.width) - 0.5) * 2;
        const py = ((viewY / config.display.height) - 0.5) * 2;
        // add particles within some safe distance from border
        if (px*px + py*py < 0.85)
            this.field.add_static_particle(new wasm.Vector(px * config.field.width, py * config.field.height))
    }

    randomField() {
        const mirrors = 3 + Math.trunc(Math.random() * 4)
        const pts = 1 + Math.trunc(mirrors / 2) + Math.trunc(Math.random() * (6 - mirrors / 2))

        for (let i = 0; i < pts; i++) {
            const th = Math.random() * Math.PI * 2;
            const r = (.2 + Math.random()) * ((i + 1) * .5 / pts) * config.field.width;
            for (let m = 0; m < mirrors; m++) {
                const x = Math.sin(th + m * 2 * Math.PI / mirrors)
                const y = Math.cos(th + m * 2 * Math.PI / mirrors)
                this.field.add_static_particle(new wasm.Vector(x * r, y * r));
            }
        }
    }
}

let app: App;

function createApp() {
    app = new App()
    return app
}

export { createApp, app }