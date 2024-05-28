<template>
    <div class="balance-table-component">
      <div class="total-staked">
        <span>Total Staked:</span>
        <span>{{ totalStaked }}</span>
      </div>
      <div class="staked-table">
        <div class="staked-row" v-for="(validator, index) in stakedValidators.slice(0, 3)" :key="index">
          <span>{{ validator.name }}:</span>
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
    data() {
      return {
        showMore: false
      };
    },
    computed: {
      totalStaked() {
        return this.stakedValidators.reduce((total, validator) => total + validator.amount, 0);
      }
    }
  };
  </script>