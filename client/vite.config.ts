import Icons from "unplugin-icons/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    Icons({
      compiler: "web-components",
      webComponents: {
        autoDefine: true,
      },
    }),
  ],
  base: "/mrt-flightradar/",
});
