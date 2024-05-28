<template>
  <div class="staking-page">
    <header class="header">
      <div class="balances d-flex justify-content-between align-items-center">
        <div class="col-md-6">
          <ToppedUpBalanceComponent :balance="toppedUpBalance" />
        </div>
        <div class="col-md-6 d-flex justify-content-end">
          <WalletBalanceComponent :balance="walletBalance" />
        </div>
      </div>
    </header>

    <div class="chain-components">
      <ChainComponent
        v-for="chain in chains"
        :key="chain.name"
        :chainName="chain.name"
        :chainImage="chain.image"
        :costToAutocompound="chain.costToAutocompound"
        :lastAutocompound="chain.lastAutocompound"
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
          name: 'Cosmos Hub',
          image: require('@/assets/chains/cosmos.svg'), 
          costToAutocompound: '0.1 ATOM',
          lastAutocompound: '1 hour ago',
          stakedValidators: [
            { name: 'Validator 1', amount: 100 },
            { name: 'Validator 2', amount: 150 },
            { name: 'Validator 3', amount: 200 },
            { name: 'Validator 4', amount: 250 }
          ],
          pendingRewards: 50
        },
        {
          name: 'Osmosis',
          image: require('@/assets/chains/osmosis.svg'),
          costToAutocompound: '0.05 OSMO',
          lastAutocompound: '2 hours ago',
          stakedValidators: [
            { name: 'Validator 1', amount: 100 },
            { name: 'Validator 2', amount: 150 }
          ],
          pendingRewards: 75
        },
        {
          name: 'Neutron',
          image: require('@/assets/chains/neutron.png'),
          costToAutocompound: '0.01 NTRN',
          lastAutocompound: '3 hours ago',
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

.header {
  .balances {
    display: flex;
    justify-content: space-between;
    align-items: center;

    .col-md-6:first-child {
      display: flex;
      align-items: center;
      justify-content: flex-start;
    }

    .col-md-6:last-child {
      display: flex;
      align-items: center;
      justify-content: flex-end;
    }
  }
}

.balance-component {
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: #f5f5f5;
  padding: 0.5rem 1rem;
  border-radius: 10px;
  text-align: center;
  margin: 0.5rem;
  min-width: 100px;
  flex-grow: 1;
}
</style>
