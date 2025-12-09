/// <reference types="vite/client" />
/** biome-ignore-all lint/complexity/noBannedTypes: vite default file */
/** biome-ignore-all lint/suspicious/noExplicitAny: vite default file */

declare module '*.vue' {
  import type { DefineComponent } from 'vue'
  const component: DefineComponent<{}, {}, any>
  export default component
}
