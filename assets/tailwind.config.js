/** @type {import('tailwindcss').Config} */

module.exports = {
  content: ["./static/**/*.html", "./views/**/*.html"],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: ["light", "dark"],
  },
};
