(self["webpackChunkvalo_html"] = self["webpackChunkvalo_html"] || []).push([["index_ts"],{

/***/ "../pkg/valo_bg.js":
/*!*************************!*\
  !*** ../pkg/valo_bg.js ***!
  \*************************/
/*! namespace exports */
/*! export Field [provided] [no usage info] [missing usage info prevents renaming] */
/*! export MovingParticle [provided] [no usage info] [missing usage info prevents renaming] */
/*! export StaticParticle [provided] [no usage info] [missing usage info prevents renaming] */
/*! export Vector [provided] [no usage info] [missing usage info prevents renaming] */
/*! export __wbg_atan2_133ce43f805276a1 [provided] [no usage info] [missing usage info prevents renaming] */
/*! export __wbg_random_c481bfb857abeff2 [provided] [no usage info] [missing usage info prevents renaming] */
/*! export __wbindgen_throw [provided] [no usage info] [missing usage info prevents renaming] */
/*! export get_random [provided] [no usage info] [missing usage info prevents renaming] */
/*! export get_random_in_range [provided] [no usage info] [missing usage info prevents renaming] */
/*! export main [provided] [no usage info] [missing usage info prevents renaming] */
/*! other exports [not provided] [no usage info] */
/*! runtime requirements: module.loaded, module.id, module, __webpack_require__.hmd, __webpack_require__, __webpack_require__.r, __webpack_exports__, __webpack_require__.d, __webpack_require__.* */
/***/ ((module, __webpack_exports__, __webpack_require__) => {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "main": () => /* binding */ main,
/* harmony export */   "get_random": () => /* binding */ get_random,
/* harmony export */   "get_random_in_range": () => /* binding */ get_random_in_range,
/* harmony export */   "Field": () => /* binding */ Field,
/* harmony export */   "MovingParticle": () => /* binding */ MovingParticle,
/* harmony export */   "StaticParticle": () => /* binding */ StaticParticle,
/* harmony export */   "Vector": () => /* binding */ Vector,
/* harmony export */   "__wbg_random_c481bfb857abeff2": () => /* binding */ __wbg_random_c481bfb857abeff2,
/* harmony export */   "__wbg_atan2_133ce43f805276a1": () => /* binding */ __wbg_atan2_133ce43f805276a1,
/* harmony export */   "__wbindgen_throw": () => /* binding */ __wbindgen_throw
/* harmony export */ });
/* harmony import */ var _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./valo_bg.wasm */ "../pkg/valo_bg.wasm");
/* module decorator */ module = __webpack_require__.hmd(module);
;

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(_valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(_valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer);
    }
    return cachegetInt32Memory0;
}

const u32CvtShim = new Uint32Array(2);

const uint64CvtShim = new BigUint64Array(u32CvtShim.buffer);
/**
*/
function main() {
    _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.main();
}

/**
* @returns {number}
*/
function get_random() {
    var ret = _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.get_random();
    return ret >>> 0;
}

/**
* @param {number} min
* @param {number} max
* @returns {number}
*/
function get_random_in_range(min, max) {
    var ret = _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.get_random_in_range(min, max);
    return ret;
}

function notDefined(what) { return () => { throw new Error(`${what} is not defined`); }; }
/**
* Renderable field
* TODO:
* 1. Multiple particle types (per bind point configuration)
* 2. (optimization) Have an "accepting" index into static particles
*    that have not exceeded their bind point limit
* 3. Global "currents" grid
* 4. (optimization) collision bins?
*/
class Field {

    static __wrap(ptr) {
        const obj = Object.create(Field.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_field_free(ptr);
    }
    /**
    * @returns {number}
    */
    get num_moving_particles() {
        var ret = _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_get_field_num_moving_particles(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set num_moving_particles(arg0) {
        _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_field_num_moving_particles(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get num_static_particles() {
        var ret = _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_get_field_num_static_particles(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set num_static_particles(arg0) {
        _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_field_num_static_particles(this.ptr, arg0);
    }
    /**
    * @param {number} half_width
    * @param {number} half_height
    */
    constructor(half_width, half_height) {
        var ret = _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.field_new(half_width, half_height);
        return Field.__wrap(ret);
    }
    /**
    * @returns {number}
    */
    moving_particles_ptr() {
        var ret = _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.field_moving_particles_ptr(this.ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    static_particles_ptr() {
        var ret = _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.field_static_particles_ptr(this.ptr);
        return ret;
    }
    /**
    * add a particle anywhere in the field
    */
    add_particle() {
        _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.field_add_particle(this.ptr);
    }
    /**
    * add a particle on the field boundary
    * @param {number} since
    */
    add_boundary_particle(since) {
        _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.field_add_boundary_particle(this.ptr, since);
    }
    /**
    * update particle positions according to time delta
    * @param {number} delta
    */
    update_positions(delta) {
        _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.field_update_positions(this.ptr, delta);
    }
    /**
    * update particle velocities according to time delta
    * @param {number} delta
    */
    update_velocities(delta) {
        _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.field_update_velocities(this.ptr, delta);
    }
    /**
    * update attachments and particles disposition
    */
    update_attachments() {
        _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.field_update_attachments(this.ptr);
    }
}
/**
* A moving particle on the field
*/
class MovingParticle {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_movingparticle_free(ptr);
    }
    /**
    * @returns {Vector}
    */
    get pos() {
        var ret = _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_get_movingparticle_pos(this.ptr);
        return Vector.__wrap(ret);
    }
    /**
    * @param {Vector} arg0
    */
    set pos(arg0) {
        _assertClass(arg0, Vector);
        var ptr0 = arg0.ptr;
        arg0.ptr = 0;
        _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_movingparticle_pos(this.ptr, ptr0);
    }
    /**
    * @returns {Vector}
    */
    get vel() {
        var ret = _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_get_movingparticle_vel(this.ptr);
        return Vector.__wrap(ret);
    }
    /**
    * @param {Vector} arg0
    */
    set vel(arg0) {
        _assertClass(arg0, Vector);
        var ptr0 = arg0.ptr;
        arg0.ptr = 0;
        _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_movingparticle_vel(this.ptr, ptr0);
    }
    /**
    * @returns {number}
    */
    get since() {
        var ret = _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_get_movingparticle_since(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set since(arg0) {
        _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_movingparticle_since(this.ptr, arg0);
    }
    /**
    * @returns {BigInt}
    */
    get flags() {
        try {
            const retptr = _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_export_0.value - 16;
            _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_export_0.value = retptr;
            _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_get_movingparticle_flags(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            u32CvtShim[0] = r0;
            u32CvtShim[1] = r1;
            const n0 = uint64CvtShim[0];
            return n0;
        } finally {
            _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_export_0.value += 16;
        }
    }
    /**
    * @param {BigInt} arg0
    */
    set flags(arg0) {
        uint64CvtShim[0] = arg0;
        const low0 = u32CvtShim[0];
        const high0 = u32CvtShim[1];
        _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_movingparticle_flags(this.ptr, low0, high0);
    }
    /**
    * @returns {number}
    */
    static get_f64_size() {
        var ret = _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.movingparticle_get_f64_size();
        return ret >>> 0;
    }
}
/**
* A static particle on the field
*/
class StaticParticle {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_staticparticle_free(ptr);
    }
    /**
    * Static particle position
    * @returns {Vector}
    */
    get pos() {
        var ret = _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_get_movingparticle_pos(this.ptr);
        return Vector.__wrap(ret);
    }
    /**
    * Static particle position
    * @param {Vector} arg0
    */
    set pos(arg0) {
        _assertClass(arg0, Vector);
        var ptr0 = arg0.ptr;
        arg0.ptr = 0;
        _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_movingparticle_pos(this.ptr, ptr0);
    }
    /**
    * Static particle rotation relative to bind configuration (degrees)
    * @returns {number}
    */
    get rot() {
        var ret = _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_get_staticparticle_rot(this.ptr);
        return ret;
    }
    /**
    * Static particle rotation relative to bind configuration (degrees)
    * @param {number} arg0
    */
    set rot(arg0) {
        _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_staticparticle_rot(this.ptr, arg0);
    }
    /**
    * Binding configuration, packed
    * [busy ports mask: u32, cfg id: u32]
    * @returns {BigInt}
    */
    get binding_cfg_id() {
        try {
            const retptr = _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_export_0.value - 16;
            _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_export_0.value = retptr;
            _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_get_staticparticle_binding_cfg_id(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            u32CvtShim[0] = r0;
            u32CvtShim[1] = r1;
            const n0 = uint64CvtShim[0];
            return n0;
        } finally {
            _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_export_0.value += 16;
        }
    }
    /**
    * Binding configuration, packed
    * [busy ports mask: u32, cfg id: u32]
    * @param {BigInt} arg0
    */
    set binding_cfg_id(arg0) {
        uint64CvtShim[0] = arg0;
        const low0 = u32CvtShim[0];
        const high0 = u32CvtShim[1];
        _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_staticparticle_binding_cfg_id(this.ptr, low0, high0);
    }
}
/**
* Vector
*/
class Vector {

    static __wrap(ptr) {
        const obj = Object.create(Vector.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_vector_free(ptr);
    }
    /**
    * @returns {number}
    */
    get x() {
        var ret = _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_get_vector_x(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set x(arg0) {
        _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_vector_x(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get y() {
        var ret = _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_get_vector_y(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set y(arg0) {
        _valo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_vector_y(this.ptr, arg0);
    }
}

const __wbg_random_c481bfb857abeff2 = typeof Math.random == 'function' ? Math.random : notDefined('Math.random');

const __wbg_atan2_133ce43f805276a1 = typeof Math.atan2 == 'function' ? Math.atan2 : notDefined('Math.atan2');

const __wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};



/***/ }),

/***/ "../pkg/valo_bg.wasm":
/*!***************************!*\
  !*** ../pkg/valo_bg.wasm ***!
  \***************************/
/*! namespace exports */
/*! export __wbg_field_free [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbg_get_field_num_moving_particles [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbg_get_field_num_static_particles [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbg_get_movingparticle_flags [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbg_get_movingparticle_pos [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbg_get_movingparticle_since [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbg_get_movingparticle_vel [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbg_get_staticparticle_binding_cfg_id [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbg_get_staticparticle_pos [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbg_get_staticparticle_rot [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbg_get_vector_x [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbg_get_vector_y [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbg_movingparticle_free [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbg_set_field_num_moving_particles [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbg_set_field_num_static_particles [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbg_set_movingparticle_flags [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbg_set_movingparticle_pos [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbg_set_movingparticle_since [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbg_set_movingparticle_vel [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbg_set_staticparticle_binding_cfg_id [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbg_set_staticparticle_pos [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbg_set_staticparticle_rot [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbg_set_vector_x [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbg_set_vector_y [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbg_staticparticle_free [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbg_vector_free [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbindgen_export_0 [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export __wbindgen_start [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export field_add_boundary_particle [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export field_add_particle [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export field_moving_particles_ptr [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export field_new [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export field_static_particles_ptr [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export field_update_attachments [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export field_update_positions [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export field_update_velocities [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export get_random [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export get_random_in_range [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export main [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export memory [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! export movingparticle_get_f64_size [provided] [no usage info] [provision prevents renaming (no use info)] */
/*! other exports [not provided] [no usage info] */
/*! runtime requirements: __webpack_require__, module, module.id, __webpack_require__.w, __webpack_require__.r, __webpack_exports__, __webpack_require__.* */
/***/ ((module, exports, __webpack_require__) => {

"use strict";
"use strict";
// Instantiate WebAssembly module
var wasmExports = __webpack_require__.w[module.id];
__webpack_require__.r(exports);
// export exports from WebAssembly module
for(var name in wasmExports) if(name) exports[name] = wasmExports[name];
// exec imports from WebAssembly module (for esm order)
/* harmony import */ var m0 = __webpack_require__(/*! ./valo_bg.js */ "../pkg/valo_bg.js");


// exec wasm module
wasmExports[""]()

/***/ }),

/***/ "./app.ts":
/*!****************!*\
  !*** ./app.ts ***!
  \****************/
/*! namespace exports */
/*! export app [provided] [no usage info] [missing usage info prevents renaming] */
/*! other exports [not provided] [no usage info] */
/*! runtime requirements: __webpack_require__, __webpack_require__.n, __webpack_require__.r, __webpack_exports__, __webpack_require__.d, __webpack_require__.* */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "app": () => /* binding */ app
/* harmony export */ });
/* harmony import */ var _pkg_valo__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ../pkg/valo */ "../pkg/valo_bg.js");
/* harmony import */ var pixi_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! pixi.js */ "pixi.js");
/* harmony import */ var pixi_js__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(pixi_js__WEBPACK_IMPORTED_MODULE_0__);
/* harmony import */ var _config__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./config */ "./config/index.ts");
/* harmony import */ var _frame__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./frame */ "./frame.ts");
;



/// app class
var App = /** @class */ (function () {
    function App() {
        this.field = new _pkg_valo__WEBPACK_IMPORTED_MODULE_3__.Field(_config__WEBPACK_IMPORTED_MODULE_1__.default.field.width, _config__WEBPACK_IMPORTED_MODULE_1__.default.field.height);
        this.pixi = new pixi_js__WEBPACK_IMPORTED_MODULE_0__.Application({
            backgroundColor: _config__WEBPACK_IMPORTED_MODULE_1__.default.colours.background,
            width: _config__WEBPACK_IMPORTED_MODULE_1__.default.display.width,
            height: _config__WEBPACK_IMPORTED_MODULE_1__.default.display.height,
            antialias: true,
        });
        this.movingParticlesContainer = new pixi_js__WEBPACK_IMPORTED_MODULE_0__.ParticleContainer(1000, { position: true, tint: true });
        this.staticParticlesContainer = new pixi_js__WEBPACK_IMPORTED_MODULE_0__.ParticleContainer(2000, { position: true, tint: true });
        this.fieldBorder = new pixi_js__WEBPACK_IMPORTED_MODULE_0__.Graphics();
        this.pixi.ticker.maxFPS = _config__WEBPACK_IMPORTED_MODULE_1__.default.display.maxfps;
        this.simulationTimeStart = (new Date()).getTime();
    }
    /// load resources
    App.prototype.load = function () {
        var _this = this;
        app.pixi.loader.add([
            { name: 'particle', url: 'images/particle.png' }
        ]).load(function () { return _this.setup(); });
    };
    /// field setup
    App.prototype.setup = function () {
        var _this = this;
        for (var i = 0; i < _config__WEBPACK_IMPORTED_MODULE_1__.default.field.numParticles; i++) {
            this.field.add_particle();
        }
        this.pixi.stage.addChild(this.movingParticlesContainer);
        this.pixi.stage.addChild(this.staticParticlesContainer);
        // set particle displays to (0,0) in the center
        this.movingParticlesContainer.setTransform(_config__WEBPACK_IMPORTED_MODULE_1__.default.display.width / 2, _config__WEBPACK_IMPORTED_MODULE_1__.default.display.height / 2, _config__WEBPACK_IMPORTED_MODULE_1__.default.display.width / (_config__WEBPACK_IMPORTED_MODULE_1__.default.field.width * 2), _config__WEBPACK_IMPORTED_MODULE_1__.default.display.height / (_config__WEBPACK_IMPORTED_MODULE_1__.default.field.height * 2));
        this.staticParticlesContainer.setTransform(_config__WEBPACK_IMPORTED_MODULE_1__.default.display.width / 2, _config__WEBPACK_IMPORTED_MODULE_1__.default.display.height / 2, _config__WEBPACK_IMPORTED_MODULE_1__.default.display.width / (_config__WEBPACK_IMPORTED_MODULE_1__.default.field.width * 2), _config__WEBPACK_IMPORTED_MODULE_1__.default.display.height / (_config__WEBPACK_IMPORTED_MODULE_1__.default.field.height * 2));
        // some basic colours
        this.movingParticlesContainer.tint = _config__WEBPACK_IMPORTED_MODULE_1__.default.colours.tintMoving;
        this.staticParticlesContainer.tint = _config__WEBPACK_IMPORTED_MODULE_1__.default.colours.tintStatic;
        // field ui
        this.fieldBorder.lineStyle(4, _config__WEBPACK_IMPORTED_MODULE_1__.default.colours.tintMoving, 1.0);
        this.fieldBorder.drawCircle(_config__WEBPACK_IMPORTED_MODULE_1__.default.display.width / 2, _config__WEBPACK_IMPORTED_MODULE_1__.default.display.height / 2, _config__WEBPACK_IMPORTED_MODULE_1__.default.display.height / 2 - 4);
        app.pixi.stage.addChild(this.fieldBorder);
        var fieldMask = new pixi_js__WEBPACK_IMPORTED_MODULE_0__.Graphics();
        fieldMask.lineStyle(0);
        fieldMask.beginFill(0xffffff);
        fieldMask.drawCircle(_config__WEBPACK_IMPORTED_MODULE_1__.default.display.width / 2, _config__WEBPACK_IMPORTED_MODULE_1__.default.display.height / 2, _config__WEBPACK_IMPORTED_MODULE_1__.default.display.height / 2 - 4);
        fieldMask.endFill();
        this.pixi.stage.mask = fieldMask;
        app.pixi.ticker.add(function (delta) { return _this.loop(delta); });
    };
    /// draw loop
    App.prototype.loop = function (delta) {
        (0,_frame__WEBPACK_IMPORTED_MODULE_2__.updateVisibleParticles)();
        for (var tick = 0; tick < _config__WEBPACK_IMPORTED_MODULE_1__.default.field.ticksPerCall; tick++) {
            this.field.update_attachments();
            this.field.update_positions(0.7);
            this.field.update_velocities(0.7);
            if (this.field.num_moving_particles + this.field.num_static_particles < _config__WEBPACK_IMPORTED_MODULE_1__.default.field.maxParticles) {
                // additional spawn rate from consumed particles
                var addSpawnRate = this.field.num_static_particles / (((new Date()).getTime() - this.simulationTimeStart) / 1000);
                // probability of spawn event happening in the last frame
                var expInterval = Math.exp(-(addSpawnRate + _config__WEBPACK_IMPORTED_MODULE_1__.default.field.spawnRate) * this.pixi.ticker.elapsedMS / 1000);
                if (Math.random() > expInterval) {
                    this.field.add_boundary_particle(this.pixi.ticker.lastTime);
                }
            }
        }
        // draw ui
        this.fieldBorder.alpha =
            (_config__WEBPACK_IMPORTED_MODULE_1__.default.field.maxParticles - this.field.num_static_particles - this.field.num_moving_particles) /
                _config__WEBPACK_IMPORTED_MODULE_1__.default.field.maxParticles;
    };
    App.prototype.reset = function () {
        this.field = new _pkg_valo__WEBPACK_IMPORTED_MODULE_3__.Field(_config__WEBPACK_IMPORTED_MODULE_1__.default.field.width, _config__WEBPACK_IMPORTED_MODULE_1__.default.field.height);
    };
    return App;
}());
var app = new App;



/***/ }),

/***/ "./config/colours.ts":
/*!***************************!*\
  !*** ./config/colours.ts ***!
  \***************************/
/*! namespace exports */
/*! export default [provided] [no usage info] [missing usage info prevents renaming] */
/*! other exports [not provided] [no usage info] */
/*! runtime requirements: __webpack_exports__, __webpack_require__.r, __webpack_require__.d, __webpack_require__.* */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "default": () => __WEBPACK_DEFAULT_EXPORT__
/* harmony export */ });
/* harmony default export */ const __WEBPACK_DEFAULT_EXPORT__ = ({
    background: 0x2f2f2f,
    text: 0xe3e3e3,
    tintMoving: 0xffdc38,
    tintStatic: 0x60e87c
});


/***/ }),

/***/ "./config/display.ts":
/*!***************************!*\
  !*** ./config/display.ts ***!
  \***************************/
/*! namespace exports */
/*! export default [provided] [no usage info] [missing usage info prevents renaming] */
/*! other exports [not provided] [no usage info] */
/*! runtime requirements: __webpack_exports__, __webpack_require__.r, __webpack_require__.d, __webpack_require__.* */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "default": () => __WEBPACK_DEFAULT_EXPORT__
/* harmony export */ });
/* harmony default export */ const __WEBPACK_DEFAULT_EXPORT__ = ({
    width: 512,
    height: 512,
    maxfps: 60
});


/***/ }),

/***/ "./config/field.ts":
/*!*************************!*\
  !*** ./config/field.ts ***!
  \*************************/
/*! namespace exports */
/*! export default [provided] [no usage info] [missing usage info prevents renaming] */
/*! other exports [not provided] [no usage info] */
/*! runtime requirements: __webpack_exports__, __webpack_require__.r, __webpack_require__.d, __webpack_require__.* */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "default": () => __WEBPACK_DEFAULT_EXPORT__
/* harmony export */ });
/* harmony default export */ const __WEBPACK_DEFAULT_EXPORT__ = ({
    width: 256,
    height: 256,
    numParticles: 0,
    maxParticles: 2000,
    spawnRate: 25.,
    ticksPerCall: 4,
});


/***/ }),

/***/ "./config/index.ts":
/*!*************************!*\
  !*** ./config/index.ts ***!
  \*************************/
/*! namespace exports */
/*! export default [provided] [no usage info] [missing usage info prevents renaming] */
/*! other exports [not provided] [no usage info] */
/*! runtime requirements: __webpack_require__, __webpack_exports__, __webpack_require__.r, __webpack_require__.d, __webpack_require__.* */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "default": () => __WEBPACK_DEFAULT_EXPORT__
/* harmony export */ });
/* harmony import */ var _colours__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./colours */ "./config/colours.ts");
/* harmony import */ var _display__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./display */ "./config/display.ts");
/* harmony import */ var _field__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./field */ "./config/field.ts");
;


/* harmony default export */ const __WEBPACK_DEFAULT_EXPORT__ = ({ colours: _colours__WEBPACK_IMPORTED_MODULE_0__.default, display: _display__WEBPACK_IMPORTED_MODULE_1__.default, field: _field__WEBPACK_IMPORTED_MODULE_2__.default });


/***/ }),

/***/ "./frame.ts":
/*!******************!*\
  !*** ./frame.ts ***!
  \******************/
/*! namespace exports */
/*! export updateVisibleParticles [provided] [no usage info] [missing usage info prevents renaming] */
/*! other exports [not provided] [no usage info] */
/*! runtime requirements: __webpack_require__, __webpack_require__.n, __webpack_require__.r, __webpack_exports__, __webpack_require__.d, __webpack_require__.* */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "updateVisibleParticles": () => /* binding */ updateVisibleParticles
/* harmony export */ });
/* harmony import */ var _pkg_valo__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ../pkg/valo */ "../pkg/valo_bg.js");
/* harmony import */ var _pkg_valo_bg_wasm__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ../pkg/valo_bg.wasm */ "../pkg/valo_bg.wasm");
/* harmony import */ var pixi_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! pixi.js */ "pixi.js");
/* harmony import */ var pixi_js__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(pixi_js__WEBPACK_IMPORTED_MODULE_0__);
/* harmony import */ var _app__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./app */ "./app.ts");
;



function addSprites(spriteContainer, numSprites) {
    for (; numSprites-- > 0;) {
        var pixie = new pixi_js__WEBPACK_IMPORTED_MODULE_0__.Sprite(_app__WEBPACK_IMPORTED_MODULE_1__.app.pixi.loader.resources.particle.texture);
        spriteContainer.addChild(pixie);
    }
}
function updateVisibleParticles() {
    regrowParticleContainer(_app__WEBPACK_IMPORTED_MODULE_1__.app.movingParticlesContainer, _app__WEBPACK_IMPORTED_MODULE_1__.app.field.num_moving_particles);
    regrowParticleContainer(_app__WEBPACK_IMPORTED_MODULE_1__.app.staticParticlesContainer, _app__WEBPACK_IMPORTED_MODULE_1__.app.field.num_static_particles);
    // update moving particles positions
    var particle_size = _pkg_valo__WEBPACK_IMPORTED_MODULE_2__.MovingParticle.get_f64_size();
    var movingParticlesView = new Float64Array(_pkg_valo_bg_wasm__WEBPACK_IMPORTED_MODULE_3__.memory.buffer, _app__WEBPACK_IMPORTED_MODULE_1__.app.field.moving_particles_ptr(), _app__WEBPACK_IMPORTED_MODULE_1__.app.field.num_moving_particles * particle_size);
    var now = _app__WEBPACK_IMPORTED_MODULE_1__.app.pixi.ticker.lastTime;
    var alpha_offset = _app__WEBPACK_IMPORTED_MODULE_1__.app.fieldBorder.alpha; // match border
    for (var i = 0; i < _app__WEBPACK_IMPORTED_MODULE_1__.app.field.num_moving_particles; i++) {
        _app__WEBPACK_IMPORTED_MODULE_1__.app.movingParticlesContainer.children[i].position.set(movingParticlesView[i * particle_size], movingParticlesView[i * particle_size + 1]);
        // moving particles have a timestamp when they appeared; we use this timestamp
        // to have particles gradually achieve full glow, 
        // and offset initial glow to match border 
        _app__WEBPACK_IMPORTED_MODULE_1__.app.movingParticlesContainer.children[i].alpha = Math.min(1.0, alpha_offset + (now - movingParticlesView[i * particle_size + 4]) / (3000));
    }
    // update static particles positions
    var staticParticlesView = new Float64Array(_pkg_valo_bg_wasm__WEBPACK_IMPORTED_MODULE_3__.memory.buffer, _app__WEBPACK_IMPORTED_MODULE_1__.app.field.static_particles_ptr(), _app__WEBPACK_IMPORTED_MODULE_1__.app.field.num_static_particles * 4);
    for (var i = 0; i < _app__WEBPACK_IMPORTED_MODULE_1__.app.field.num_static_particles; i++) {
        _app__WEBPACK_IMPORTED_MODULE_1__.app.staticParticlesContainer.children[i].position.set(staticParticlesView[i * 4], staticParticlesView[i * 4 + 1]);
    }
}
function regrowParticleContainer(spriteContainer, actualCount) {
    if (spriteContainer.children.length < actualCount) {
        // add more sprites
        addSprites(spriteContainer, actualCount - spriteContainer.children.length);
    }
    else if (spriteContainer.children.length > actualCount) {
        // remove sprites
        spriteContainer.removeChildren(actualCount);
    }
}



/***/ }),

/***/ "./index.ts":
/*!******************!*\
  !*** ./index.ts ***!
  \******************/
/*! namespace exports */
/*! exports [not provided] [no usage info] */
/*! runtime requirements: __webpack_require__, __webpack_require__.r, __webpack_exports__, __webpack_require__.* */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony import */ var _app__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./app */ "./app.ts");
var _a;

_app__WEBPACK_IMPORTED_MODULE_0__.app.load();
/// bind app to DOM
(_a = document.getElementById("view")) === null || _a === void 0 ? void 0 : _a.appendChild(_app__WEBPACK_IMPORTED_MODULE_0__.app.pixi.view);
window.setInterval(function () {
    var elFps = document.getElementById('fps');
    if (elFps)
        elFps.innerHTML = _app__WEBPACK_IMPORTED_MODULE_0__.app.pixi.ticker.FPS.toFixed(0);
}, 667);


/***/ })

}]);