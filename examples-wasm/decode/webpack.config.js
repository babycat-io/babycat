const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  mode: 'production',
  experiments: {
    asyncWebAssembly: true,
    topLevelAwait: true,
  },
  entry: {
    index: './src/index.js',
  },
  plugins: [
    new HtmlWebpackPlugin({
      title: 'Index',
      filename: 'index.html',
    }),
  ],
  output: {
    filename: '[name].bundle.js',
    path: path.resolve(__dirname, '../../target/release/examples-wasm/decode/'),
  },
};
