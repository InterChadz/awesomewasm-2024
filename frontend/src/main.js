// Vue.js
import {createApp} from "vue";
import App from "@/App.vue";
import router from "@/router";
import store from "./store";
import {BootstrapIconsPlugin} from "bootstrap-icons-vue";

// Bootstrap framework
import "bootstrap/dist/css/bootstrap.min.css";
import "bootstrap";

// Vue toasted
import Toast, {POSITION} from "vue-toastification";

createApp(App)
  .use(router)
  .use(store)
  .use(BootstrapIconsPlugin)
  .use(Toast, {
    position: POSITION.BOTTOM_RIGHT,
    maxToasts: 3
  })
  .mount("#app");
