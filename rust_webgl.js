/* tslint:disable */
import * as wasm from './rust_webgl_bg';
import { test } from './js/glue';
import { _get_webgl_rendering_context } from './js/glue';

const __wbg_log_13505707723175b6_target = console.log;

const TextDecoder = typeof self === 'object' && self.TextDecoder
    ? self.TextDecoder
    : require('util').TextDecoder;

let cachedDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

export function __wbg_log_13505707723175b6(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    __wbg_log_13505707723175b6_target(varg0);
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

function getArrayU32FromWasm(ptr, len) {
    return getUint32Memory().subarray(ptr / 4, ptr / 4 + len);
}

export function __wbg_test_ea3b28c5d4373174(arg0, arg1) {
    let varg0 = getArrayU32FromWasm(arg0, arg1);
    test(varg0);
}

const stack = [];

const slab = [{ obj: undefined }, { obj: null }, { obj: true }, { obj: false }];

function getObject(idx) {
    if ((idx & 1) === 1) {
        return stack[idx >> 1];
    } else {
        const val = slab[idx >> 1];
        
        return val.obj;
        
    }
}

let slab_next = slab.length;

function addHeapObject(obj) {
    if (slab_next === slab.length) slab.push(slab.length + 1);
    const idx = slab_next;
    const next = slab[idx];
    
    slab_next = next;
    
    slab[idx] = { obj, cnt: 1 };
    return idx << 1;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

export function __wbg__get_webgl_rendering_context_c92030751bae6ef9(arg0) {
    
    const val = _get_webgl_rendering_context(getObject(arg0));
    return isLikeNone(val) ? 0 : addHeapObject(val);
    
}

const __wbg_createShader_263451728040c7be_target = WebGLRenderingContext.prototype.createShader  || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.createShader does not exist`);
} ;

export function __wbg_createShader_263451728040c7be(arg0, arg1) {
    return addHeapObject(__wbg_createShader_263451728040c7be_target.call(getObject(arg0), arg1));
}

const __wbg_shaderSource_8ba5948aaba5a941_target = WebGLRenderingContext.prototype.shaderSource  || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.shaderSource does not exist`);
} ;

export function __wbg_shaderSource_8ba5948aaba5a941(arg0, arg1, arg2, arg3) {
    let varg2 = getStringFromWasm(arg2, arg3);
    __wbg_shaderSource_8ba5948aaba5a941_target.call(getObject(arg0), getObject(arg1), varg2);
}

const __wbg_compileShader_4eb2fd3e6abeef67_target = WebGLRenderingContext.prototype.compileShader  || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.compileShader does not exist`);
} ;

export function __wbg_compileShader_4eb2fd3e6abeef67(arg0, arg1) {
    __wbg_compileShader_4eb2fd3e6abeef67_target.call(getObject(arg0), getObject(arg1));
}

const __wbg_getShaderInfoLog_a4499dc05c84b28c_target = WebGLRenderingContext.prototype.getShaderInfoLog  || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.getShaderInfoLog does not exist`);
} ;

const TextEncoder = typeof self === 'object' && self.TextEncoder
    ? self.TextEncoder
    : require('util').TextEncoder;

let cachedEncoder = new TextEncoder('utf-8');

function passStringToWasm(arg) {
    
    const buf = cachedEncoder.encode(arg);
    const ptr = wasm.__wbindgen_malloc(buf.length);
    getUint8Memory().set(buf, ptr);
    return [ptr, buf.length];
}

export function __wbg_getShaderInfoLog_a4499dc05c84b28c(ret, arg0, arg1) {
    
    const [retptr, retlen] = passStringToWasm(__wbg_getShaderInfoLog_a4499dc05c84b28c_target.call(getObject(arg0), getObject(arg1)));
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;
    
}

const __wbg_createProgram_7b4b055f3f5a0c66_target = WebGLRenderingContext.prototype.createProgram  || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.createProgram does not exist`);
} ;

export function __wbg_createProgram_7b4b055f3f5a0c66(arg0) {
    return addHeapObject(__wbg_createProgram_7b4b055f3f5a0c66_target.call(getObject(arg0)));
}

const __wbg_attachShader_9defe6b18fc7326a_target = WebGLRenderingContext.prototype.attachShader  || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.attachShader does not exist`);
} ;

export function __wbg_attachShader_9defe6b18fc7326a(arg0, arg1, arg2) {
    __wbg_attachShader_9defe6b18fc7326a_target.call(getObject(arg0), getObject(arg1), getObject(arg2));
}

const __wbg_linkProgram_3ce7bd3987b47968_target = WebGLRenderingContext.prototype.linkProgram  || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.linkProgram does not exist`);
} ;

export function __wbg_linkProgram_3ce7bd3987b47968(arg0, arg1) {
    __wbg_linkProgram_3ce7bd3987b47968_target.call(getObject(arg0), getObject(arg1));
}

const __wbg_getProgramParameter_8927e423b26cb276_target = WebGLRenderingContext.prototype.getProgramParameter  || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.getProgramParameter does not exist`);
} ;

export function __wbg_getProgramParameter_8927e423b26cb276(arg0, arg1, arg2) {
    return __wbg_getProgramParameter_8927e423b26cb276_target.call(getObject(arg0), getObject(arg1), arg2) ? 1 : 0;
}

const __wbg_createBuffer_64c11cb8bdcbaa23_target = WebGLRenderingContext.prototype.createBuffer  || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.createBuffer does not exist`);
} ;

export function __wbg_createBuffer_64c11cb8bdcbaa23(arg0) {
    return addHeapObject(__wbg_createBuffer_64c11cb8bdcbaa23_target.call(getObject(arg0)));
}

const __wbg_deleteBuffer_dac31bbf04257393_target = WebGLRenderingContext.prototype.deleteBuffer  || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.deleteBuffer does not exist`);
} ;

export function __wbg_deleteBuffer_dac31bbf04257393(arg0, arg1) {
    __wbg_deleteBuffer_dac31bbf04257393_target.call(getObject(arg0), getObject(arg1));
}

const __wbg_bindBuffer_be58f2bfa54a23d5_target = WebGLRenderingContext.prototype.bindBuffer  || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.bindBuffer does not exist`);
} ;

export function __wbg_bindBuffer_be58f2bfa54a23d5(arg0, arg1, arg2) {
    __wbg_bindBuffer_be58f2bfa54a23d5_target.call(getObject(arg0), arg1, getObject(arg2));
}

const __wbg_bufferData_3feaadc9f48faf9f_target = WebGLRenderingContext.prototype.bufferData  || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.bufferData does not exist`);
} ;

let cachegetFloat32Memory = null;
function getFloat32Memory() {
    if (cachegetFloat32Memory === null || cachegetFloat32Memory.buffer !== wasm.memory.buffer) {
        cachegetFloat32Memory = new Float32Array(wasm.memory.buffer);
    }
    return cachegetFloat32Memory;
}

function getArrayF32FromWasm(ptr, len) {
    return getFloat32Memory().subarray(ptr / 4, ptr / 4 + len);
}

export function __wbg_bufferData_3feaadc9f48faf9f(arg0, arg1, arg2, arg3, arg4) {
    let varg2 = getArrayF32FromWasm(arg2, arg3);
    __wbg_bufferData_3feaadc9f48faf9f_target.call(getObject(arg0), arg1, varg2, arg4);
}

const __wbg_VERTEX_SHADER_9b64bf514a8fd9e3_target = function() {
    return WebGLRenderingContext.VERTEX_SHADER;
}  ;

export function __wbg_VERTEX_SHADER_9b64bf514a8fd9e3() {
    return __wbg_VERTEX_SHADER_9b64bf514a8fd9e3_target();
}

const __wbg_FRAGMENT_SHADER_10f8f66781b1d336_target = function() {
    return WebGLRenderingContext.FRAGMENT_SHADER;
}  ;

export function __wbg_FRAGMENT_SHADER_10f8f66781b1d336() {
    return __wbg_FRAGMENT_SHADER_10f8f66781b1d336_target();
}

const __wbg_ARRAY_BUFFER_a745e64ed12e8949_target = function() {
    return WebGLRenderingContext.ARRAY_BUFFER;
}  ;

export function __wbg_ARRAY_BUFFER_a745e64ed12e8949() {
    return __wbg_ARRAY_BUFFER_a745e64ed12e8949_target();
}

const __wbg_STATIC_DRAW_acf790fde85b497a_target = function() {
    return WebGLRenderingContext.STATIC_DRAW;
}  ;

export function __wbg_STATIC_DRAW_acf790fde85b497a() {
    return __wbg_STATIC_DRAW_acf790fde85b497a_target();
}

const __wbg_LINK_STATUS_f1d3ed9958a634cc_target = function() {
    return WebGLRenderingContext.LINK_STATUS;
}  ;

export function __wbg_LINK_STATUS_f1d3ed9958a634cc() {
    return __wbg_LINK_STATUS_f1d3ed9958a634cc_target();
}

function addBorrowedObject(obj) {
    stack.push(obj);
    return ((stack.length - 1) << 1) | 1;
}
/**
* @param {any} arg0
* @param {string} arg1
* @returns {void}
*/
export function test_get_context(arg0, arg1) {
    const [ptr1, len1] = passStringToWasm(arg1);
    try {
        return wasm.test_get_context(addBorrowedObject(arg0), ptr1, len1);
        
    } finally {
        stack.pop();
        wasm.__wbindgen_free(ptr1, len1 * 1);
        
    }
    
}

/**
* @param {string} arg0
* @returns {void}
*/
export function greet(arg0) {
    const [ptr0, len0] = passStringToWasm(arg0);
    try {
        return wasm.greet(ptr0, len0);
        
    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);
        
    }
    
}

function dropRef(idx) {
    
    idx = idx >> 1;
    if (idx < 4) return;
    let obj = slab[idx];
    
    obj.cnt -= 1;
    if (obj.cnt > 0) return;
    
    // If we hit 0 then free up our space in the slab
    slab[idx] = slab_next;
    slab_next = idx;
}

export function __wbindgen_object_drop_ref(i) {
    dropRef(i);
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

