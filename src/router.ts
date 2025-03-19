import { createWebHistory, createRouter } from "vue-router";

import HomePage from "./views/HomePage.vue";
import CompetitionPage from "./views/CompetitionPage.vue";
import SimulationPage from "./views/SimulationPage.vue";
import CustomPage from "./views/CustomPage.vue";

const routes = [
  { path: "/", component: HomePage },
  { path: "/competition/:id", component: CompetitionPage },
  { path: "/simulation/", component: SimulationPage },
  { path: "/custom/", component: CustomPage },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
