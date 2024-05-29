<template>
  <button class="btn btn-link p-0" @click.prevent="onClickConnect" v-if="!userSigner">
    <span>Connect Wallet</span>
  </button>
  <button class="btn btn-link p-0" v-else>
    <span>{{ userAddress ? userAddress.substring(0, 10) : 'Error' }}...</span>
  </button>
</template>

<script>
import {mapActions, mapGetters} from "vuex";
import mxApp from "@/mixin/app";

export default {
  name: "WalletComponent",

  mixins: [mxApp],

  computed: {
    ...mapGetters(["userSigner", "userAddress"])
  },

  methods: {
    ...mapActions(['initUser', 'fetchUserData', "fetchUserDelegations"]),

    async onClickConnect() {
      await this.initUser()
      await this.fetchUserData();
      await this.fetchUserDelegations()
    }
  }
}
</script>

<style lang="scss" scoped>
@import "@/assets/style.scss";

.btn, .btn:hover, .btn:focus {
  text-transform: uppercase;
}
</style>