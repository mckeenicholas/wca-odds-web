import { createWebHistory, createRouter } from "vue-router";

import Home from "./views/Home.vue";
import Competition from "./views/Competition.vue";
import Simulation from "./views/Simulation.vue";
import Custom from "./views/Custom.vue";
// import Test from "./views/Test.vue";

const routes = [
  { path: "/", component: Home },
  { path: "/competition/:id", component: Competition },
  { path: "/simulation/", component: Simulation },
  { path: "/custom/", component: Custom },
  // { path: "/beta/", component: Test },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
