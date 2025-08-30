/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{rs,html,css}",
    "./src/**/*.rs",
  ],
  theme: {
    extend: {
      colors: {
        neon: {
          pink: '#ff0080',
          blue: '#0080ff',
          green: '#00ff80',
          yellow: '#ffff00',
          purple: '#8000ff',
        }
      }
    },
  },
  plugins: [],
}
