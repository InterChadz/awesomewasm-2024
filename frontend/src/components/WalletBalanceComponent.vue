<template>
  <div class="balance-component">
    <p><b>Wallet Balance: </b>{{ balance }} <CoinComponent /></p>
  </div>
</template>

<script>
import CoinComponent from "@/components/Common/CoinComponent.vue";


export default {
  name: 'WalletBalanceComponent',
  components: {
    CoinComponent
  },
  props: {
    balance: Number
  }
};
</script>

<style scoped>
.balance-component {
  display: flex;
  align-items: center;
}
</style>
