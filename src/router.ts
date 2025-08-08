import { createRouter, createWebHistory } from "vue-router";

const HomePage = () => import("./views/HomePage.vue");
const CompetitionPage = () => import("./views/CompetitionPage.vue");
const SimulationPage = () => import("./views/SimulationPage.vue");
const CustomPage = () => import("./views/CustomPage.vue");

const routes = [
  { path: "/", component: HomePage },
  { path: "/competition/:id", component: CompetitionPage },
  { path: "/competition/:id/results/", component: SimulationPage },
  { path: "/custom/", component: CustomPage },
  { path: "/custom/results/", component: SimulationPage },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
