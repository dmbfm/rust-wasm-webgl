require('./js/vao-polyfill')

const js = import('./rust_webgl')

// Check if a variable is a function
function isFunction(functionToCheck) {
  return functionToCheck && {}.toString.call(functionToCheck) === '[object Function]';
}

// Shim WebGLRendering context to debug all calls
for (let key of Object.getOwnPropertyNames(WebGLRenderingContext.prototype)) {
    let val
    try {
        val = WebGLRenderingContext.prototype[key]
    } catch(e) {
        break;
    }


    if (isFunction(val)) {
        let oldFunc = val
        WebGLRenderingContext.prototype[key] = function(...args) {
            console.log(`start:WebGLRenderingContext#${key}(${args})`)
            let ret = val.apply(this, args)
            console.log(`end:WebGLRenderingContext#${key} -> ${ret}`)
            return ret
        }
    }
}

js.then(js => {
    js.greet("Rust")
     let canvas = document.createElement('canvas')
     document.body.appendChild(canvas)
     js.test_get_context(canvas, 'webgl')
})