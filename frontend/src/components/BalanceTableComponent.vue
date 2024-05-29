<template>
  <div class="balance-table-component">
    <div class="total-staked">
      <span>Total Staked:</span>
      <span>{{ totalStaked }}</span>
    </div>
    <div class="staked-table">
      <div class="staked-row" v-for="(validator, index) in stakedValidators.slice(0, 3)" :key="index">
        <span>{{ shortenAddress(validator.address) }}:</span>
        <span>{{ validator.amount }}</span>
      </div>
    </div>
    <div class="pending-rewards">
      <span>Pending Rewards:</span>
      <span>{{ pendingRewards }}</span>
    </div>
  </div>
</template>

<script>
export default {
  name: 'BalanceTableComponent',
  props: {
    stakedValidators: Array,
    pendingRewards: Number
  },
  computed: {
    totalStaked() {
      return this.stakedValidators.reduce((total, validator) => total + validator.amount, 0);
    }
  },
  methods: {
    shortenAddress(address) {
      return address.slice(0, 6) + '...' + address.slice(-4);
    }
  }
};
</script>