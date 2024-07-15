const path = require("path");
const fs = require("fs");
const HtmlWebpackPlugin = require("html-webpack-plugin");

module.exports = {
  mode: "development",
  devtool: "source-map",
  context: __dirname + "/src",
  entry: {
    main: "./main.js",
    worker: "./worker.js",
  },
  output: {
    filename: "[name].[contenthash].js",
    path: path.resolve(__dirname, "dist"),
    clean: true,
  },
  plugins: [new HtmlWebpackPlugin()],
  devServer: {
    compress: true,
    port: 8081,
    server: {
      type: "https",
      options: {
        key: fs.readFileSync("dev-server.local.pem"),
        cert: fs.readFileSync("dev-server.local.pem"),
      },
    },
    headers: {
      "Cross-Origin-Embedder-Policy": "require-corp",
      "Cross-Origin-Opener-Policy": "same-origin",
    },
  },
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
  experiments: {
    asyncWebAssembly: true,
  },
};
