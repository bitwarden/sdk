const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  plugins: [
    new HtmlWebpackPlugin(),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "."),
    }),
  ],
  optimization: {
    splitChunks: {
      cacheGroups: {
        commons: {
          test: /[\\/]node_modules[\\/]/,
          name: "app/vendor",
          chunks: (chunk) => {
            return chunk.name === "app/main";
          },
        },
      },
    },
  },
  mode: "development",
  experiments: {
    asyncWebAssembly: true,
  },
};
