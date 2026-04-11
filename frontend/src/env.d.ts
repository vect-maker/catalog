interface ImportMetaEnv {
  readonly VITE_API_URL: string;
  readonly VITE_STORE_NAME: string;
  readonly VITE_STORE_PHONE_NUMBER: string;
}
declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}