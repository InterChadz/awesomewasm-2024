<template>
  <div class="balance-table-component">
    <div class="card" v-for="(item, index) in userFilteredDelegations" :key="index">
      <div class="card-body">
        <h5>Your delegations</h5>

        <div v-for="(delegation, index) in item.delegations" :key="index">
          <ul class="list-unstyled">
            <li>Validator: {{ delegation.delegation.validator_address.substring(0, 20) }}...</li>
            <li>Delegated: {{ displayAmount(delegation.balance.amount, 2) }}
              {{ delegation.balance.denom.substring(1).toUpperCase() }}
            </li>
          </ul>

          <div v-if="userFilteredRewards.length">
            <h5>Pending Rewards</h5>
            <div v-for="(rewards, rindex) in userFilteredRewards" :key="rindex">
              <ul class="list-unstyled">
                <li
                  v-for="(reward, zindex) in rewards.calculated_reward.rewards.find(i => i.validator === delegation.delegation.validator_address).reward"
                  :key="zindex">
                  {{ displayAmount(reward.amount, 2) }} {{ reward.denom.substring(1).toUpperCase() }}
                </li>
              </ul>
            </div>
          </div>
          <p v-else>To start autocompounding, you have register the user on this chain first.</p>

          <hr/>

          <ButtonComponent v-if="!grants.length" text="Grant" class="btn btn-primary" :is-small="true"
                           @click.prevent="wrapperGrant(delegation.delegation.validator_address)"/>
          <ButtonComponent v-else text="Revoke" class="btn btn-primary" :is-small="true"
                           @click.prevent="wrapperRevoke(delegation.delegation.validator_address)"/>
        </div>
        <p v-if="!item.delegations.length">You have no delegations!</p>
      </div>
    </div>

    <p class="text-end mt-3">Total staked: {{ displayAmount(totalStaked, 2) }}
      <CoinComponent/>
    </p>
  </div>
</template>

<script>
import mxChain from '@/mixin/chain';
import {mapGetters} from "vuex";
import CoinComponent from "@/components/Common/CoinComponent.vue";
import ButtonComponent from "@/components/Common/ButtonComponent.vue";
import {QueryClient, setupAuthzExtension} from "@cosmjs/stargate";
import {Tendermint34Client} from "@cosmjs/tendermint-rpc";
import mxToast from "@/mixin/toast";

export default {
  name: 'BalanceTableComponent',
  components: {ButtonComponent, CoinComponent},

  mixins: [mxChain, mxToast],

  props: {
    chain: {
      type: Object,
      required: true
    }
  },

  data() {
    return {
      grants: []
    }
  },

  created() {
    this.fetchGrants()
  },

  computed: {
    ...mapGetters(['userDelegations', 'userRewards', 'userSigners']),

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
  },

  methods: {
    async wrapperGrant(valAddress) {
      try {
        const response = await this.grantAuthZ(this.userSigners.find(s => s.chainId === this.chain.chain_id).address, this.chain.ica_address, [valAddress])
        console.log(response)
        await this.fetchGrants()
        this.toast.success("Permission successfully granted.")
      } catch (e) {
        this.toast.error("Failed granting permission.")
        console.error(e)
      }
    },

    async wrapperRevoke(valAddress) {
      try {
        const response = await this.revokeAuthZ(this.userSigners.find(s => s.chainId === this.chain.chain_id).address, this.chain.ica_address, [valAddress])
        console.log(response)
        await this.fetchGrants()
        this.toast.success("Permission successfully revoked.")
      } catch (e) {
        this.toast.error("Failer revoking permission.")
        console.error(e)
      }
    },

    async fetchGrants() {
      // grant permission is always the ICA address, but for a different validator addy foreach staking position
      const apis = JSON.parse(process.env.VUE_APP_CHAINS_APIS);
      const api = apis.find(api => api.chainId === this.chain.chain_id);
      console.log("found authz RPC for client to external chains", api.rpc);

      const tmClient = await Tendermint34Client.connect(api.rpc)

      const queryClient = QueryClient.withExtensions(tmClient, setupAuthzExtension);
      console.log("QUERY CLIENT WITH EXTENSIONS", queryClient)

      // TODO: Do the query to AuthZç
      // TODO: gaiad q authz grants-by-granter cosmos10h9stc5v6ntgeygf5xf945njqq5h32r53uquvw --node tcp://:16657
      const response = await queryClient.authz.granterGrants(this.userSigners.find(s => s.chainId === this.chain.chain_id).address);
      console.log("response.grants", response.grants)
      this.grants = response.grants
    }
  }
};
</script>