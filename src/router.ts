import { createWebHistory, createRouter } from "vue-router";

import Home from "./views/Home.vue";
import Competition from "./views/Competition.vue";
import Simulation from "./views/Simulation.vue";

const routes = [
  { path: "/", component: Home },
  { path: "/competition/:id", component: Competition },
  { path: "/simulation/", component: Simulation },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
