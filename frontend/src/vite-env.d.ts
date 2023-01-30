/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_WEBAPP: string;
  readonly VITE_DEFAULT_ENDPOINT: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
