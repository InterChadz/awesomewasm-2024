<template>
  <div id="app-body" class="d-flex flex-column">
    <LoadingComponent v-if="isBusy"/>

    <template v-else>
      <NavbarComponent/>

      <div class="main-section flex-grow-1">
        <router-view/>
      </div>

      <FooterComponent/>
    </template>
  </div>
</template>

<script>
import NavbarComponent from "@/components/Layout/NavbarComponent.vue";
import LoadingComponent from "@/components/Common/LoadingComponent.vue";
import FooterComponent from "@/components/Layout/FooterComponent.vue";
import mxApp from "@/mixin/app";
import {mapGetters} from "vuex";
import mxChain from "@/mixin/chain";

export default {
  name: "App",

  mixins: [mxApp, mxChain],

  components: {FooterComponent, LoadingComponent, NavbarComponent},

  computed: {
    ...mapGetters(['appConfig'])
  },

  data() {
    return {
      isBusy: true,
      intervalTimeout: Number(process.env.VUE_APP_INTERVAL_TIMEOUT)
    }
  },

  async created() {
    try {
      await this.suggestChain()
    } catch (e) {
      //console.error(e)
    }
    await this.fetchOnce();
    // we ensure that till this moment rest of UI is kept idle
    this.isBusy = false;
  },

  unmounted() {
    if (this.intervalId) clearInterval(this.intervalId)
  }
};
</script>

<style lang="scss">
@import "@/assets/style.scss";
</style>