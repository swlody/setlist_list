const path = require("path");

const distPath = path.resolve(__dirname, "static/dist");
const fs = require("fs");
if (!fs.existsSync(distPath)) {
  fs.mkdirSync(distPath);
}

module.exports = {
  entry: "./src/index.js",
  output: {
    filename: "main.js",
    path: distPath,
  },
};
