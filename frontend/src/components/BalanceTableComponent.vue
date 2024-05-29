<template>
  <div class="balance-table-component">
    <div class="total-staked">
      <span>Total Staked:</span>
      <span>{{ displayAmount(totalStaked) }}</span>
    </div>
    <div class="staked-table">
      <div class="staked-row" v-for="(validator, index) in stakedValidators.slice(0, 3)" :key="index">
        <span>{{ shortenAddress(validator.address) }}:</span>
        <span>{{ displayAmount(validator.amount) }}</span>
      </div>
    </div>
    <div class="pending-rewards">
      <span>Pending Rewards:</span>
      <span>{{ displayAmount(pendingRewards) }}</span>
    </div>
  </div>
</template>

<script>
import mxChain from '@/mixin/chain';

export default {
  name: 'BalanceTableComponent',
  mixins:[mxChain],
  props: {
    stakedValidators: Array,
    totalStaked: Number,
    pendingRewards: Number
  },
  methods: {
    shortenAddress(address) {
      return address.slice(0, 6) + '...' + address.slice(-4);
    }
  }
};
</script>