const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const webpack = require("webpack");

module.exports = {
  entry: "./index.ts",
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
      {
        test: /\.wasm$/,
        type: "webassembly/sync",
      },
    ],
  },
  resolve: {
    extensions: [".tsx", ".ts", ".js"],
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  plugins: [
    new HtmlWebpackPlugin(),
    // Have this example work in Edge which doesn't ship `TextEncoder` or
    // `TextDecoder` at this time.
    new webpack.ProvidePlugin({
      TextDecoder: ["text-encoding", "TextDecoder"],
      TextEncoder: ["text-encoding", "TextEncoder"],
    }),
  ],
  experiments: {
    syncWebAssembly: true,
  },
  devServer: {
    proxy: {
      "/api": {
        target: "http://localhost:4000",
        pathRewrite: { "^/api": "" },
        secure: false,
        changeOrigin: true,
      },
      "/identity": {
        target: "http://localhost:33656",
        pathRewrite: { "^/identity": "" },
        secure: false,
        changeOrigin: true,
      },
    },
  },
  mode: "development",
  devtool: "source-map",
};
