import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        blurple: "#5865F2",
        green: "#57F287",
        red: "#ED4245",
        white: "#FFFFFF",
        background: "#1D1D21",
        backgroundSide: "#121214",
        hoverSide: "#2C2C30",
        gray: {
          gray: "#adadad",
          50: "#68696e",
          100: "#4E5058",
          200: "#36393F",
          300: "#2F3136",
          400: "#222327",

        },
      },
    },
  },
  plugins: [],
};
export default config;
