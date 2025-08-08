import { VueQueryPlugin, VueQueryPluginOptions } from "@tanstack/vue-query";
import { createPinia } from "pinia";
import { createApp } from "vue";
import App from "./App.vue";
import "./assets/index.css";
import router from "./router";

const vueQueryPluginOptions: VueQueryPluginOptions = {
  queryClientConfig: {
    defaultOptions: {
      queries: {
        refetchOnWindowFocus: false,
        staleTime: Infinity,
      },
    },
  },
};

const pinia = createPinia();

const app = createApp(App)
  .use(router)
  .use(pinia)
  .use(VueQueryPlugin, vueQueryPluginOptions);

app.config.performance = true;

app.mount("#app");
