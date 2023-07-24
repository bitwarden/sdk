const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const webpack = require("webpack");

module.exports = {
  entry: "./src/bench.ts",
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
    filename: "bench.js",
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "../../crates/bitwarden-wasm"),
      outDir: path.resolve(__dirname, "../../languages/nodejs_wasm/pkg"),
      extraArgs: "--target nodejs --all-features",
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
  mode: "production",
  devtool: "source-map",
  target: "node",
};

