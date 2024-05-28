import {createRouter, createWebHistory} from "vue-router";

const routes = [
  {
    path: "/",
    name: "Restaker",
    meta: {
      title: "Restaker"
    },
    component: () => import("@/views/Home"),
  },
  {
    path: "/team",
    name: "Team",
    meta: {
      title: "Our Team",
    },
    component: () => import("@/views/Team"),
  },
  {
    path: "/staking",
    name: "Staking",
    meta: {
      title: "Dashboard",
    },
    component: () => import("@/views/Staking"),
  },
  {
    path: "/:pathMatch(.*)*",
    name: "Uups!",
    meta: {
      title: "Uups!",
      error: 404,
    },
    component: () => import("@/views/Error"),
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

export default router;
