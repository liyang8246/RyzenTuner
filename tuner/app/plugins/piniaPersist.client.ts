/* eslint-disable @typescript-eslint/no-explicit-any */
import type { Pinia, PiniaPluginContext } from 'pinia'

declare module 'pinia' {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  export interface DefineStoreOptionsBase<S, Store> {
    persist?: boolean
    afterRestore?: (store: Store) => void
  }
}

interface StorageAdapter {
  read: (key: string) => Promise<any>
  write: (key: string, value: any) => Promise<void>
}

class TauriStorageAdapter implements StorageAdapter {
  async read(key: string): Promise<any> {
    return await invoke<string | null>('storage_read', { key })
  }

  async write(key: string, value: any): Promise<void> {
    return await invoke('storage_write', { key, value })
  }
}

async function piniaPersist({ store, options }: PiniaPluginContext) {
  if (!options.persist) return

  const storage = new TauriStorageAdapter()
  const key = `pinia-${store.$id}`

  const data = await storage.read(key)
  if (data) store.$patch(JSON.parse(data))
  options.afterRestore?.(store)

  store.$subscribe(() => {
    storage.write(key, JSON.stringify(store.$state))
  })

}

export default defineNuxtPlugin((nuxtApp) => {
  const $pinia = nuxtApp.$pinia as Pinia
  $pinia.use(piniaPersist)
})
