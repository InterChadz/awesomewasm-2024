<template>
  <div class="staking-page">
    <div v-if="loading" class="loading">
      Loading data, please wait...
    </div>
    <div v-else-if="error" class="error">
      <p>Failed to load data. Please try again.</p>
      <button @click="retryFetchData">Retry</button>
    </div>
    <div v-else>
      <div class="balances d-flex">
        <div class="col-md-4 d-flex">
          <ToppedUpBalanceComponent :balance="toppedUpBalance" />
        </div>
        <div class="col-md-4 d-flex"> </div>
        <div class="col-md-4 d-flex">
          <WalletBalanceComponent :balance="walletBalance" />
        </div>
      </div>

      <div class="chain-components">
        <ChainComponent
          v-for="chain in filteredChains"
          :key="chain.name"
          :chainName="chain.name"
          :chainImage="chain.image"
          :costToAutocompound="chain.costToAutocompound"
          :lastAutocompound="chain.lastAutocompound"
          :stakedValidators="chain.stakedValidators"
          :totalStaked="chain.totalStaked"
          :pendingRewards="chain.pendingRewards"
        />
      </div>
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
    ...mapGetters(['userBalance', 'userContractBalance', 'userRegistrations', 'userRewards', 'appSupportedChains']),
    walletBalance() {
      return this.userBalance;
    },
    toppedUpBalance() {
      return this.userContractBalance;
    },
    filteredChains() {
      const supportedChainIds = this.appSupportedChains.map(chain => chain.chain_id);
      return this.chains.map(chain => {
        const isSupported = supportedChainIds.includes(chain.chainId);
        if (isSupported) {
          const registration = this.userRegistrations.find(reg => reg.chain_id === chain.chainId);
          const reward = this.userRewards.find(reward => reward.chain_id === chain.chainId);
          return {
            ...chain,
            stakedValidators: registration ? registration.validators.map(validator => ({
              address: validator,
              amount: 0
            })) : [],
            totalStaked: reward ? reward.calculated_reward.total_delegation : 0,
            pendingRewards: reward ? reward.calculated_reward.reward : 0
          };
        }
        return null;
      }).filter(chain => chain !== null);
    }
  },
  data() {
    return {
      chains: [
        {
          name: 'Cosmos Hub',
          chainId: 'test-0',
          image: require('@/assets/chains/cosmos.svg'),
          costToAutocompound: '0.1 ATOM',
          lastAutocompound: '1 hour ago'
        },
        {
          name: 'Osmosis',
          chainId: 'test-1',
          image: require('@/assets/chains/osmosis.svg'),
          costToAutocompound: '0.05 OSMO',
          lastAutocompound: '2 hours ago'
        },
        {
          name: 'Neutron',
          chainId: 'test-2',
          image: require('@/assets/chains/neutron.png'),
          costToAutocompound: '0.01 NTRN',
          lastAutocompound: '3 hours ago'
        }
      ]
    };
  },
  created() {
    this.$store.dispatch('fetchAppSupportedChains');
    this.$store.dispatch('fetchUserData');
  }
};
</script>

<style lang="scss" scoped>
@import "@/assets/style.scss";
</style>
