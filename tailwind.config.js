/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        bg: {
          DEFAULT: "#0a0a0a",
          sidebar: "#111111",
          panel: "#1a1a1a",
        },
        accent: "#7c5cff",
        border: "#222222",
        txt: {
          DEFAULT: "#e5e5e5",
          secondary: "#888888",
        },
      },
      fontFamily: {
        sans: ["Inter", "system-ui", "sans-serif"],
      },
    },
  },
  plugins: [],
};
