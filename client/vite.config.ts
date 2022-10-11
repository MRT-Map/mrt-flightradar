import { defineConfig } from "vite";
import Icons from 'unplugin-icons/vite'

export default defineConfig({
  plugins: [
    Icons({
      compiler: "web-components",
      webComponents: {
        autoDefine: true,
      },
    })
  ],
})