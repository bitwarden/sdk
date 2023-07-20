const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const webpack = require("webpack");

module.exports = {
  entry: "./index.ts",
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: [".ts", ".js"],
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: "./index.html",
      filename: "index.html",
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "../../crates/bitwarden-wasm"),
      outDir: path.resolve(__dirname, "../../languages/js_webassembly/pkg"),
      extraArgs: "-- --all-features",
      forceMode: "production",
    }),
    new webpack.ProvidePlugin({
      TextDecoder: ["text-encoding", "TextDecoder"],
      TextEncoder: ["text-encoding", "TextEncoder"],
    }),
  ],
  experiments: {
    asyncWebAssembly: true,
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
    port: 8081,
  },
  mode: "development",
  devtool: "source-map",
};
