/** @type {import('tailwindcss').Config} */

const path = require("path");
const distPath = path.resolve(__dirname, "static/dist");
const fs = require("fs");
if (!fs.existsSync(distPath)) {
  fs.mkdirSync(distPath);
}

const staticPath = path.resolve(__dirname, "static");
const viewsPath = path.resolve(__dirname, "views");

module.exports = {
  content: [`${staticPath}/**/*.html`, `${viewsPath}/**/*.html`],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui"), require("@tailwindcss/typography")],
  daisyui: {
    themes: ["light", "dark"],
  },
};
