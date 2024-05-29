<template>
  <div class="balance-table-component">
    <div class="card" v-for="(item, index) in userFilteredDelegations" :key="index">
      <div class="card-body">
        <h5>Your delegations</h5>

        <div v-for="(delegation, index) in item.delegations" :key="index">
          <ul class="list-unstyled">
            <li>Validator: {{delegation.delegation.validator_address.substring(0, 20)}}...</li>
            <li>Delegated: {{displayAmount(delegation.balance.amount, 2)}} {{ delegation.balance.denom.substring(1).toUpperCase()}}</li>
          </ul>

          <h5>Pending Rewards</h5>
          <div v-for="(rewards, rindex) in userFilteredRewards" :key="rindex">
            <ul class="list-unstyled">
              <li v-for="(reward, zindex) in rewards.calculated_reward.rewards.find(i => i.validator === delegation.delegation.validator_address).reward" :key="zindex">
                {{displayAmount(reward.amount, 2)}} {{reward.denom.substring(1).toUpperCase()}}
              </li>
            </ul>
          </div>
        </div>
      </div>
    </div>

    <hr/>

    <p class="text-end">Total staked: {{ displayAmount(totalStaked, 2) }} <CoinComponent/></p>
  </div>
</template>

<script>
import mxChain from '@/mixin/chain';
import {mapGetters} from "vuex";
import CoinComponent from "@/components/Common/CoinComponent.vue";

export default {
  name: 'BalanceTableComponent',
  components: {CoinComponent},

  mixins: [mxChain],

  props: {
    chain: {
      type: Object,
      required: true
    }
  },

  computed: {
    ...mapGetters(['userDelegations', 'userRewards']),

    userFilteredDelegations() {
      return this.userDelegations.filter(delegation => delegation.chain_id === this.chain.chain_id);
    },

    userFilteredRewards() {
      return this.userRewards.filter(reward => reward.chain_id === this.chain.chain_id);
    },

    totalStaked() {
      return this.userFilteredDelegations.reduce((acc, item) => {
        return acc + item.delegations.reduce((acc, delegation) => {
          return acc + parseInt(delegation.balance.amount);
        }, 0);
      }, 0);
    },
  }
};
</script>