const path = require('path')
const UglifyJsPlugin = require('uglifyjs-webpack-plugin')

module.exports = {
    entry: path.resolve(__dirname, 'index.js'),
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
        publicPath: '/dist/'

    },
    mode: 'development',
    // optimization: {
    //     minimizer: [
    //         new UglifyJsPlugin()
    //     ]
    // }
}