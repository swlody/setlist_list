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
  plugins: [
    require("daisyui"),
    require("@tailwindcss/forms"),
    require("@tailwindcss/typography"),
    require("@tailwindcss/aspect-ratio"),
    require("@tailwindcss/line-clamp"),
  ],
  daisyui: {
    themes: ["light", "dark"],
  },
};
