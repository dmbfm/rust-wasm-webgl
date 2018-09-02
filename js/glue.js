export const _get_webgl_rendering_context = canvas => canvas.getContext('webgl')

export const _get_2d_rendering_context = canvas => canvas.getContext('2d')

export const test = x => { console.log('test', x) }

export const _get_ext_oes_vertex_array_object = gl => gl.getExtension('OES_vertex_array_object');

export const _create_vertex_array_oes = ext => {
   console.log('start:_create_vertex_array_oes')
   let val = ext.createVertexArrayOES()
   console.log('end:_create_vertex_array_oes -> ' + val)
   return val
}

export const _bind_vertex_array_oes = (ext, vao) => {
    console.log('start:_bind_vertex_array_oes ' + vao)
    let val = ext.bindVertexArrayOES(vao)
    console.log('end:_bind_vertex_array_oes -> ' + val)
}

export const _delete_vertex_array_oes = (ext, vao) => {
 console.log('start:_delete_vertex_array_oes' + vao)
 let val = ext.deleteVertexArrayOES(vao)
 console.log('end:_delete_vertex_array_oes -> ' + val)
}

