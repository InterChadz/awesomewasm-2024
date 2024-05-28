<template>

  <div class="container">
    <div class="row">
      <div class="col-12">
        Content here
      </div>
    </div>
  </div>
</template>

<script>
import {mapGetters} from "vuex";
import mxApp from "@/mixin/app";

export default {
  name: 'HomeView',

  mixins: [mxApp],

  computed: {
    ...mapGetters(['appConfig', 'appState'])
  }
};
</script>

<style lang="scss" scoped>
@import "@/assets/style.scss";
</style>