/* tslint:disable */
import * as wasm from './rust_webgl_bg';
import { test } from './js/glue';
import { _get_webgl_rendering_context } from './js/glue';
import { _get_ext_oes_vertex_array_object } from './js/glue';
import { _create_vertex_array_oes } from './js/glue';
import { _bind_vertex_array_oes } from './js/glue';

const __wbg_log_067e41a04467204b_target = console.log;

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

export function __wbg_log_067e41a04467204b(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    __wbg_log_067e41a04467204b_target(varg0);
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

export function __wbg_test_0f75046ff9963a64(arg0, arg1) {
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

export function __wbg_getwebglrenderingcontext_c43683ed0a6e2946(arg0) {
    
    const val = _get_webgl_rendering_context(getObject(arg0));
    return isLikeNone(val) ? 0 : addHeapObject(val);
    
}

const __wbg_createShader_06e89541e425ef34_target = WebGLRenderingContext.prototype.createShader  || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.createShader does not exist`);
} ;

export function __wbg_createShader_06e89541e425ef34(arg0, arg1) {
    return addHeapObject(__wbg_createShader_06e89541e425ef34_target.call(getObject(arg0), arg1));
}

const __wbg_shaderSource_1c906cd2dda44819_target = WebGLRenderingContext.prototype.shaderSource  || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.shaderSource does not exist`);
} ;

export function __wbg_shaderSource_1c906cd2dda44819(arg0, arg1, arg2, arg3) {
    let varg2 = getStringFromWasm(arg2, arg3);
    __wbg_shaderSource_1c906cd2dda44819_target.call(getObject(arg0), getObject(arg1), varg2);
}

const __wbg_compileShader_abd5128cc5c91681_target = WebGLRenderingContext.prototype.compileShader  || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.compileShader does not exist`);
} ;

export function __wbg_compileShader_abd5128cc5c91681(arg0, arg1) {
    __wbg_compileShader_abd5128cc5c91681_target.call(getObject(arg0), getObject(arg1));
}

const __wbg_getShaderInfoLog_429d134bc1271803_target = WebGLRenderingContext.prototype.getShaderInfoLog  || function() {
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

export function __wbg_getShaderInfoLog_429d134bc1271803(ret, arg0, arg1) {
    
    const [retptr, retlen] = passStringToWasm(__wbg_getShaderInfoLog_429d134bc1271803_target.call(getObject(arg0), getObject(arg1)));
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;
    
}

const __wbg_createProgram_7b3dfcae0bff5536_target = WebGLRenderingContext.prototype.createProgram  || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.createProgram does not exist`);
} ;

export function __wbg_createProgram_7b3dfcae0bff5536(arg0) {
    return addHeapObject(__wbg_createProgram_7b3dfcae0bff5536_target.call(getObject(arg0)));
}

const __wbg_attachShader_7ab36187798812f4_target = WebGLRenderingContext.prototype.attachShader  || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.attachShader does not exist`);
} ;

export function __wbg_attachShader_7ab36187798812f4(arg0, arg1, arg2) {
    __wbg_attachShader_7ab36187798812f4_target.call(getObject(arg0), getObject(arg1), getObject(arg2));
}

const __wbg_linkProgram_6080c97357fefb11_target = WebGLRenderingContext.prototype.linkProgram  || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.linkProgram does not exist`);
} ;

export function __wbg_linkProgram_6080c97357fefb11(arg0, arg1) {
    __wbg_linkProgram_6080c97357fefb11_target.call(getObject(arg0), getObject(arg1));
}

const __wbg_getProgramParameter_02d7992ca938244a_target = WebGLRenderingContext.prototype.getProgramParameter  || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.getProgramParameter does not exist`);
} ;

export function __wbg_getProgramParameter_02d7992ca938244a(arg0, arg1, arg2) {
    return __wbg_getProgramParameter_02d7992ca938244a_target.call(getObject(arg0), getObject(arg1), arg2) ? 1 : 0;
}

const __wbg_createBuffer_3b7cb9af2f2f80c8_target = WebGLRenderingContext.prototype.createBuffer  || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.createBuffer does not exist`);
} ;

export function __wbg_createBuffer_3b7cb9af2f2f80c8(arg0) {
    return addHeapObject(__wbg_createBuffer_3b7cb9af2f2f80c8_target.call(getObject(arg0)));
}

const __wbg_deleteBuffer_e7ab2b93de9537b6_target = WebGLRenderingContext.prototype.deleteBuffer  || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.deleteBuffer does not exist`);
} ;

export function __wbg_deleteBuffer_e7ab2b93de9537b6(arg0, arg1) {
    __wbg_deleteBuffer_e7ab2b93de9537b6_target.call(getObject(arg0), getObject(arg1));
}

const __wbg_bindBuffer_35e0eba75d85cfef_target = WebGLRenderingContext.prototype.bindBuffer  || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.bindBuffer does not exist`);
} ;

export function __wbg_bindBuffer_35e0eba75d85cfef(arg0, arg1, arg2) {
    __wbg_bindBuffer_35e0eba75d85cfef_target.call(getObject(arg0), arg1, getObject(arg2));
}

const __wbg_bufferData_019685d6f7ae4b7c_target = WebGLRenderingContext.prototype.bufferData  || function() {
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

export function __wbg_bufferData_019685d6f7ae4b7c(arg0, arg1, arg2, arg3, arg4) {
    let varg2 = getArrayF32FromWasm(arg2, arg3);
    __wbg_bufferData_019685d6f7ae4b7c_target.call(getObject(arg0), arg1, varg2, arg4);
}

const __wbg_enableVertexAttribArray_31a9186b2f4b39fc_target = WebGLRenderingContext.prototype.enableVertexAttribArray  || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.enableVertexAttribArray does not exist`);
} ;

export function __wbg_enableVertexAttribArray_31a9186b2f4b39fc(arg0, arg1) {
    __wbg_enableVertexAttribArray_31a9186b2f4b39fc_target.call(getObject(arg0), arg1);
}

const __wbg_vertexAttribPointer_0d170b8075bcadc0_target = WebGLRenderingContext.prototype.vertexAttribPointer  || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.vertexAttribPointer does not exist`);
} ;

export function __wbg_vertexAttribPointer_0d170b8075bcadc0(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    __wbg_vertexAttribPointer_0d170b8075bcadc0_target.call(getObject(arg0), arg1, arg2, arg3, arg4 !== 0, arg5, arg6);
}

export function __wbg_getextoesvertexarrayobject_79bf18a07ee0c4de(arg0) {
    
    const val = _get_ext_oes_vertex_array_object(getObject(arg0));
    return isLikeNone(val) ? 0 : addHeapObject(val);
    
}

const __wbg_VERTEXSHADER_6821061559ba378e_target = function() {
    return WebGLRenderingContext.VERTEX_SHADER;
}  ;

export function __wbg_VERTEXSHADER_6821061559ba378e() {
    return __wbg_VERTEXSHADER_6821061559ba378e_target();
}

const __wbg_FRAGMENTSHADER_6ab4af2c9b9e619f_target = function() {
    return WebGLRenderingContext.FRAGMENT_SHADER;
}  ;

export function __wbg_FRAGMENTSHADER_6ab4af2c9b9e619f() {
    return __wbg_FRAGMENTSHADER_6ab4af2c9b9e619f_target();
}

const __wbg_ARRAYBUFFER_587dcee7f75d4fb4_target = function() {
    return WebGLRenderingContext.ARRAY_BUFFER;
}  ;

export function __wbg_ARRAYBUFFER_587dcee7f75d4fb4() {
    return __wbg_ARRAYBUFFER_587dcee7f75d4fb4_target();
}

const __wbg_STATICDRAW_c87e431c2aa4bfb5_target = function() {
    return WebGLRenderingContext.STATIC_DRAW;
}  ;

export function __wbg_STATICDRAW_c87e431c2aa4bfb5() {
    return __wbg_STATICDRAW_c87e431c2aa4bfb5_target();
}

const __wbg_LINKSTATUS_fda0e5e51bef3fbc_target = function() {
    return WebGLRenderingContext.LINK_STATUS;
}  ;

export function __wbg_LINKSTATUS_fda0e5e51bef3fbc() {
    return __wbg_LINKSTATUS_fda0e5e51bef3fbc_target();
}

const __wbg_FLOAT_545ecf8057c65f3c_target = function() {
    return WebGLRenderingContext.FLOAT;
}  ;

export function __wbg_FLOAT_545ecf8057c65f3c() {
    return __wbg_FLOAT_545ecf8057c65f3c_target();
}

export function __wbg_createvertexarrayoes_885b73910c84e685(arg0) {
    return addHeapObject(_create_vertex_array_oes(getObject(arg0)));
}

export function __wbg_bindvertexarrayoes_16a68136c94d5007(arg0, arg1) {
    _bind_vertex_array_oes(getObject(arg0), getObject(arg1));
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

