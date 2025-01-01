/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      // placeholder: {
      //   italic: {
      //     "font-style": "italic",
      //   },
      // },
      fontFamily: {
        montserrat: ['"Montserrat"', "sans-serif"],
        lexend: ['"Lexend"', "sans-serif"],
      },
      backgroundImage: {
        "grid-pattern": `
            linear-gradient(to right, rgba(230, 230, 230, 0.7) 1px, transparent 1px),
            linear-gradient(to bottom, rgba(230, 230, 230, 0.7) 1px, transparent 1px)
        `,
      },
      backgroundSize: {
        "grid-pattern": "20px 20px",
      },
    },
  },
  plugins: [],
};
