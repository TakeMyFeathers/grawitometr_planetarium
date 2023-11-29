/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    screens:{
      'lg': '2000px'
    },
    extend: {
      fontFamily: {
        'bellefair':['Bellefair', 'serif']
      }
    },
  },
  plugins: [],
}

