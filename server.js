const express = require('express')

const app = express()

express.static.mime.types['wasm'] = 'application/wasm'

app.use(express.static('.'))

app.listen(8080, () => {
    console.log('Listening...')
})