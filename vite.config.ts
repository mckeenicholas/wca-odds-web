import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import tailwind from "tailwindcss";
import autoprefixer from "autoprefixer";
import path from "node:path";
import { AcceptedPlugin } from "postcss";

// https://vitejs.dev/config/
export default defineConfig({
  css: {
    postcss: {
      // This is probably really, really bad, but I honestly can't find a better
      // way to do it, and the app wont build otherwise until the chart dependencies
      // are updated to support tailwind v4
      plugins: [tailwind() as AcceptedPlugin, autoprefixer()],
    },
  },
  plugins: [vue()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  worker: {
    format: "es",
  },
});
