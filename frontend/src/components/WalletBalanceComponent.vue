<template>
  <div class="balance-component">
    <span><b>Wallet Balance: </b>{{ balance }} <CoinComponent /></span>
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
