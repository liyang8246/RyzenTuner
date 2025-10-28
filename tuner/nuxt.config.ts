// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },
  css: ['~/assets/main.css'],
  ssr: false,
  modules: ['@nuxt/ui', '@nuxt/eslint', '@pinia/nuxt'],
  vite: {
    clearScreen: false,
    envPrefix: ['VITE_', 'TAURI_'],
  },
  ignore: ['**/src-tauri/**'],
  imports: {
    dirs: ['types'],
    presets: [{
      from: '@tauri-apps/api/core',
      imports: ['invoke'],
    }]
  }
})