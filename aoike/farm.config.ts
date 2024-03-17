import { defineConfig } from "@farmfe/core";
import solid from "vite-plugin-solid";

export default defineConfig({
  vitePlugins: [() => ({
    vitePlugin: solid({extensions: [".md"]}), filters: [
      '\\.jsx$',
      '\\.tsx$',
      '\\.md$',
    ]
  })],
  plugins: ["farm-plugin-aoike"]
});
