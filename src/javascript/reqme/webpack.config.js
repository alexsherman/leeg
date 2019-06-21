const HtmlWebPackPlugin = require("html-webpack-plugin");
var path = require('path');

module.exports = {
  module: {
    rules: [
      {
        test: /\.(js|jsx)$/,
        exclude: /node_modules/,
        use: {
          loader: "babel-loader"
        }
      },
      {
        test: /\.html$/,
        use: [
          {
            loader: "html-loader"
          }
        ]
      },
      {
        test: /\.scss$/,
        use: [
              'style-loader', 
              'css-loader', 
              'sass-loader'
        ]
      },
    ]
  },
  plugins: [
    new HtmlWebPackPlugin({
      template: "src/index.html",
    })
  ],
  devServer: {
    historyApiFallback: true
  },
  output: {
    filename: 'index_bundle.js',
  }
};