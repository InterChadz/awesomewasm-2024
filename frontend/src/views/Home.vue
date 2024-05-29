<template>
  <div class="landing-page">
    <header class="header">
      <img src="@/assets/logo.png" alt="Logo" class="logo" />
      <h1>InterChadz</h1>
    </header>
    <section class="description">
      <p>
        Real chads restake on InterChadz. 
      </p>
    </section>

    <section class="features">
    <div class="container-fluid">
      <div class="row">
        <div class="col-md-4">
          <div class="card bg-secondary border-0 rounded-3 p-3">
            <h2>Auto-compound staking</h2>
            <p>
              InterChadz is available on multiple chains. By auto-compounding all your staked amounts periodically, InterChadz helps you to maximize your staking yield.
            </p>
          </div>
        </div>
        <div class="col-md-4">
          <div class="card bg-secondary border-0 rounded-3 p-3">
            <h2>One time registration</h2>
            <p>
              Users on InterChadz only have to register their wallet once to benefit from continuous auto-compounding on the platform.
            </p>
          </div>
        </div>
        <div class="col-md-4">
          <div class="card bg-secondary border-0 rounded-3 p-3">
            <h2>Fees</h2>
            <p>
              All fees and charges on are handled by the InterChadz platform. The dashboard gives you a clear view of all the fees, charges and rewards.
            </p>
          </div>
        </div>
      </div>
    </div>
    </section>

    <section class="team">    <TeamComponent /></section>

  </div>
</template>

<script>
import { mapGetters } from "vuex";
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
