<template>
  <div class="container mb-5">
    <div class="row">
      <div class="col-12 text-center">
        <img src="@/assets/logo.png" alt="Logo" class="logo"/>
        <h1>InterChadz</h1>
        <p>Real chads restake on Interchain</p>
      </div>
    </div>
  </div>

  <div class="container mb-5">
    <div class="row text-center">
      <div class="col-md-4">
        <div class="card bg-secondary border-0 rounded-3 p-3">
          <h2 class="text-white">Auto-compound staking</h2>
          <p class="text-white">
            InterChadz is available on multiple chains. By auto-compounding all your staked amounts periodically, it
            helps you to maximize your staking yield.
          </p>
        </div>
      </div>
      <div class="col-md-4">
        <div class="card bg-secondary border-0 rounded-3 p-3">
          <h2 class="text-white">One time registration</h2>
          <p class="text-white">
            Users on InterChadz only have to register their wallet once per supported chain to benefit from continuous
            auto-compounding on the platform.
          </p>
        </div>
      </div>
      <div class="col-md-4">
        <div class="card bg-secondary border-0 rounded-3 p-3">
          <h2 class="text-white">Fees</h2>
          <p class="text-white">
            All fees and charges on are handled by the InterChadz platform. The dashboard gives you a clear view of all
            the fees, charges and rewards.
          </p>
        </div>
      </div>
    </div>
  </div>


  <div class="container">
    <div class="row">
      <div class="col-12">
        <TeamComponent/>
      </div>
    </div>
  </div>
</template>

<script>
import {mapGetters} from "vuex";
import mxApp from "@/mixin/app";
import TeamComponent from '@/components/TeamComponent.vue';

export default {
  name: 'HomeView',

  mixins: [mxApp],

  components: {
    TeamComponent
  },
  computed: {
    ...mapGetters(['appConfig', 'appState'])
  }
};
</script>

<style lang="scss" scoped>
@import "@/assets/style.scss";
</style>
