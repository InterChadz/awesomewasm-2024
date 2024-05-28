<template>
  <div class="landing-page">
    <header class="header">
      <img src="@/assets/logo.png" alt="Logo" class="logo" />
      <h1>InterChadz</h1>
    </header>
    <section class="description">
      <p>
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
      </p>
    </section>

    <section class="features">
    <div class="container-fluid">
      <div class="row">
        <div class="col-md-4">
          <div class="card bg-secondary border-0 rounded-3 p-3">
            <h2>Card 1</h2>
            <p>
              Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
            </p>
          </div>
        </div>
        <div class="col-md-4">
          <div class="card bg-secondary border-0 rounded-3 p-3">
            <h2>Card 2</h2>
            <p>
              Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
            </p>
          </div>
        </div>
        <div class="col-md-4">
          <div class="card bg-secondary border-0 rounded-3 p-3">
            <h2>Card 2</h2>
            <p>
              Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
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
