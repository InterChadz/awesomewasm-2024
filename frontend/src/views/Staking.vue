<template>
  <div class="staking-page">
    <div>
      <div class="balances d-flex">
        <div class="col-md-4 d-flex">
          <ToppedUpBalanceComponent/>
        </div>
        <div class="col-md-4 d-flex"></div>
        <div class="col-md-4 d-flex">
          <WalletBalanceComponent/>
        </div>
      </div>

      <div class="chain-components">
        <ChainComponent v-for="(chain, index) in appSupportedChains" :key="index"
                        :chain="chain"
        />
      </div>
    </div>
  </div>
</template>

<script>
import {mapGetters} from 'vuex';
import ToppedUpBalanceComponent from '@/components/ToppedUpBalanceComponent.vue';
import WalletBalanceComponent from '@/components/WalletBalanceComponent.vue';
import ChainComponent from '@/components/ChainComponent.vue';

export default {
  name: 'StakingView',

  components: {
    ToppedUpBalanceComponent,
    WalletBalanceComponent,
    ChainComponent
  },

  computed: {
    ...mapGetters(['userBalance', 'userContractBalance', 'userRegistrations', 'userRewards', 'appSupportedChains', "userAddress"]),
  }
};
</script>

<style lang="scss" scoped>
@import "@/assets/style.scss";
</style>
