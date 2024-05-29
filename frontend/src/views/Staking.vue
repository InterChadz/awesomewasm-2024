<template>
  <div class="staking-page">
    <div class="balances d-flex ">
      <div class="col-md-4 d-flex">
        <ToppedUpBalanceComponent :balance="toppedUpBalance" />
      </div>
      <div class="col-md-4 d-flex"> </div>
      <div class="col-md-4 d-flex ">
        <WalletBalanceComponent :balance="walletBalance" />
      </div>
    </div>

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
import { mapGetters } from 'vuex';
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
  computed: {
    ...mapGetters(['userBalance', 'userContractBalance', 'supportedChains']),
    walletBalance() {
      return this.userBalance;
    },
    toppedUpBalance() {
      return this.userContractBalance; 
    },
    // chains() {
    //   return this.supportedChains;
    // }
  },
  data() {
    return {
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
</style>
