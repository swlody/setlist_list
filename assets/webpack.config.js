const path = require("path");
const distPath = path.resolve(__dirname, "static/dist");
const fs = require("fs");
if (!fs.existsSync(distPath)) {
  fs.mkdirSync(distPath);
}

const srcDir = path.resolve(__dirname, "src");

module.exports = {
  entry: `${srcDir}/index.js`,
  output: {
    filename: "main.js",
    path: distPath,
  },
};
