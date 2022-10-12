import Icons from "unplugin-icons/vite";
import { defineConfig } from "vite";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
  plugins: [
    Icons({
      compiler: "web-components",
      webComponents: {
        autoDefine: true,
      },
    }),
    topLevelAwait(),
  ],
  base: "/mrt-flightradar/",
});
