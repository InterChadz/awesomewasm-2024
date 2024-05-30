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
        <div class="action-buttons p-3">
          <button @click="compound">Compound</button> {{dueDelegationsAmount}}
        </div>

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
import mxToast from "@/mixin/toast";
import mxChain from "@/mixin/chain";

export default {
  name: 'StakingView',

  mixins: [mxToast, mxChain],

  components: {
    ToppedUpBalanceComponent,
    WalletBalanceComponent,
    ChainComponent
  },

  computed: {
    ...mapGetters(['appSupportedChains', "appDueUserRegistrations", 'appConfig']),

    // The amount is the sum of all delegations from all the users registered among all the supported chains
    // If a user has more than one delegation in the same chain, the amount will be the sum of all the delegations, but not amount, just number of delegations.
    dueDelegationsAmount() {
      // [
      //   {
      //     "local_address":"neutron10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf",
      //     "chain_id":"test-2",
      //     "remote_address":"cosmos10h9stc5v6ntgeygf5xf945njqq5h32r53uquvw",
      //     "validators":[
      //       "cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn"
      //     ],
      //     "delegator_delegations_reply_id":1,
      //     "delegator_delegations_icq_id":1,
      //     "next_compound_height":126
      //   }
      // ]
      return this.appDueUserRegistrations.reduce((acc, item) => {
        return acc + item.validators.length;
      }, 0);
    },

    keeperReward() {
      if (!this.dueDelegationsAmount) return 0

      return this.dueDelegationsAmount
    }
  },

  methods: {
    async compound() {
      try {
        await this.autocompound();
        this.toast.success("Rewards compounded successfully.");
      } catch (error) {
        console.error("failed to compound", error)
        this.toast.error("Failed to compound rewards.");
      }
    }
  }
};
</script>

<style lang="scss" scoped>
@import "@/assets/style.scss";
</style>
