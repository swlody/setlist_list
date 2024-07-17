/** @type {import('tailwindcss').Config} */

const path = require("path");
const distPath = path.resolve(__dirname, "assets/static/dist");
const fs = require("fs");
if (!fs.existsSync(distPath)) {
  fs.mkdirSync(distPath);
}

module.exports = {
  content: ["./assets/static/**/*.html", "./assets/views/**/*.html"],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: ["light", "dark"],
  },
};
