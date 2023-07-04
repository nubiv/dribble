/** @type {import('tailwindcss').Config} */
module.exports = {
    content: { 
      files: ["*.html", "./src/**/**"],
    },
    theme: {
      extend: {
          colors: {
              'primary': '#1E1E1E',
            },
      },
    },
    plugins: [],
  }