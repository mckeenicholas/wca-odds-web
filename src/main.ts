import { createApp } from "vue";
import router from "./router";
import "./assets/index.css";
import App from "./App.vue";
import { VueQueryPlugin, VueQueryPluginOptions } from "@tanstack/vue-query";

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

const app = createApp(App)
  .use(router)
  .use(VueQueryPlugin, vueQueryPluginOptions);

app.config.performance = true;

app.mount("#app");
