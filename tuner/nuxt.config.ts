// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },
  ssr: false,
  modules: ['@nuxt/ui', '@nuxt/eslint'],
  vite: {
    clearScreen: false,
    envPrefix: ['VITE_', 'TAURI_'],
  },
  ignore: ['**/src-tauri/**'],
})