<template>
  <div class="staking-page">
    <header class="header">
      <div class="balances d-flex justify-content-between align-items-stretch">
        <div class="col-md-6">
          <ToppedUpBalanceComponent title="Topped Up Balance" :balance="toppedUpBalance" />
        </div>
        <div class="col-md-6 d-flex justify-content-end">
          <WalletBalanceComponent title="Wallet Balance" :balance="walletBalance" />
        </div>
      </div>
    </header>

    <div class="chain-components">
      <ChainComponent
        v-for="chain in chains"
        :key="chain.name"
        :chainName="chain.name"
        :chainDetails="chain.details"
        :stakedValidators="chain.stakedValidators"
        :pendingRewards="chain.pendingRewards"
      />
    </div>
  </div>
</template>

<script>
import ToppedUpBalanceComponent from '@/components/ToppedUpBalanceComponent.vue';
import WalletBalanceComponent from '@/components/WalletBalanceComponent.vue';
import ChainComponent from '@/components/ChainComponent.vue';

export default {
  name: 'StakingPage',
  components: {
    ToppedUpBalanceComponent,
    WalletBalanceComponent,
    ChainComponent
  },
  data() {
    return {
      toppedUpBalance: 1000, // Example data, replace with actual data
      walletBalance: 2000, // Example data, replace with actual data
      chains: [
        {
          name: 'Chain 1',
          details: 'Details about Chain 1',
          stakedValidators: [
            { name: 'Validator 1', amount: 100 },
            { name: 'Validator 2', amount: 150 },
            { name: 'Validator 3', amount: 200 },
            { name: 'Validator 4', amount: 250 }
          ],
          pendingRewards: 50
        },
        {
          name: 'Chain 2',
          details: 'Details about Chain 2',
          stakedValidators: [
            { name: 'Validator 1', amount: 100 },
            { name: 'Validator 2', amount: 150 }
          ],
          pendingRewards: 75
        },
        {
          name: 'Chain 3',
          details: 'Details about Chain 3',
          stakedValidators: [
            { name: 'Validator 1', amount: 100 }
          ],
          pendingRewards: 25
        }
      ]
    };
  }
};
</script>

<style lang="scss" scoped>
@import "@/assets/style.scss";

</style>
