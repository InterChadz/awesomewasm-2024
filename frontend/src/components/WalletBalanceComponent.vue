<template>
  <div class="balance-component">
    <strong class="me-1">Wallet Balance:</strong> {{ userBalance }} <CoinComponent class="ms-1"/>
  </div>
</template>

<script>
import CoinComponent from "@/components/Common/CoinComponent.vue";
import {mapGetters} from "vuex";

export default {
  name: 'WalletBalanceComponent',

  components: {
    CoinComponent
  },

  computed: {
    ...mapGetters(['userBalance'])
  }
};
</script>

<style scoped>
.balance-component {
  display: flex;
  align-items: center;
}
</style>
