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
        gray: {
          100: "#4E5058",
          200: "#36393F",
          300: "#2F3136",
          400: "#202225",
          dark: "#c85555",
        },
      },
    },
  },
  plugins: [],
};
export default config;
