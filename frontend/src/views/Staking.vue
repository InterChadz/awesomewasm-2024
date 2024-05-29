<template>
  <div class="staking-page">
    <div>
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
          :isActive="chain.isActive"
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
      console.log("Supported Chain IDs:", supportedChainIds);
      console.log("User Registrations:", this.userRegistrations);
      
      return this.chains.map(chain => {
        const isSupported = supportedChainIds.includes(chain.chainId);
        console.log(`Chain ${chain.chainId} is supported: ${isSupported}`);
        
        if (isSupported) {
          const registration = this.userRegistrations.find(reg => reg.chain_id === chain.chainId);
          console.log(`Chain ${chain.chainId} registration:`, registration);
          
          const reward = this.userRewards.find(reward => reward.chain_id === chain.chainId);
          console.log(`Chain ${chain.chainId} reward:`, reward);
          
          return {
            ...chain,
            stakedValidators: registration ? registration.validators.map(validator => ({
              address: validator,
              amount: 0
            })) : [],
            totalStaked: reward ? reward.calculated_reward.total_delegation : 0,
            pendingRewards: reward ? reward.calculated_reward.reward : 0,
            isActive: !!registration
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
          name: 'Osmosis',
          chainId: 'test-0',
          image: require('@/assets/chains/osmosis.svg'),
          costToAutocompound: '0.05 OSMO',
          lastAutocompound: '2 hours ago'
        },
        {
          name: 'Neutron',
          chainId: 'test-1',
          image: require('@/assets/chains/neutron.png'),
          costToAutocompound: '0.01 NTRN',
          lastAutocompound: '3 hours ago'
        },
        {
          name: 'Cosmos Hub',
          chainId: 'test-2',
          image: require('@/assets/chains/cosmos.svg'),
          costToAutocompound: '0.1 ATOM',
          lastAutocompound: '1 hour ago'
        }
      ]
    };
  },
  methods: {
    async fetchData() {
      this.error = false;
      try {
        await this.$store.dispatch('fetchAppSupportedChains');
        await this.$store.dispatch('fetchUserData');
      } catch (error) {
        console.error("Error fetching data:", error);
      }
    },
    retryFetchData() {
      this.retryCount = 0;
      this.fetchData();
    }
  },
  created() {
    this.fetchData();
  }
};
</script>

<style lang="scss" scoped>
@import "@/assets/style.scss";

</style>
