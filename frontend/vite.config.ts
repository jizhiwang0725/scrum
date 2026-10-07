import path from "path"
import react from '@vitejs/plugin-react'
import { defineConfig } from 'vite'
import tailwindcss from '@tailwindcss/vite'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    react(),
    tailwindcss(),
  ],

  resolve: {
    // 8. alias: Crucial configuration for setting up path aliases!
    alias: {
      // 9. Maps the "@" symbol to the absolute path of your local "src" directory.
      // Example: import { Button } from "@/components/Button"
      "@": path.resolve(__dirname, "./src"),
    },
  },
})


