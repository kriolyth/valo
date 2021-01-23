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

import { app } from './app';

app.load();

// bind app to DOM
document.getElementById("view")?.appendChild(app.pixi.view);

document.getElementById("reset")?.addEventListener("click", () => {
    app.reset()
    let pp = document.getElementById("play-pause");
    if (pp) {
        pp.innerHTML = 'Play'
    }
})
document.getElementById("play-pause")?.addEventListener("click", () => {
    let pp = document.getElementById("play-pause");
    if (!pp)
        return;
    if (app.isReady() && app.isPaused()) {
        app.resume();
        pp.innerHTML = 'Pause'
    }
    else if (app.isReady() && !app.isPaused()) {
        app.pause();
        pp.innerHTML = 'Play'
    } else if (!app.isReady()) {
        app.start();
        pp.innerHTML = 'Pause'
    }
})

window.setInterval(() => {
    const elFps = document.getElementById('fps');
    if (elFps)
        elFps.innerHTML = app.pixi.ticker.FPS.toFixed(0);
}, 667);

