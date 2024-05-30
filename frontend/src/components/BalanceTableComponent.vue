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

          <ButtonComponent v-if="!isValidatorGranted(delegation.delegation.validator_address)" text="Grant / Revoke" class="btn btn-primary" :is-small="true"
                           @click.prevent="grantAuthZ(userAddress, chain.ica_address, [delegation.delegation.validator_address])"/>
          <ButtonComponent v-else text="Revoke" class="btn btn-primary" :is-small="true"
                           @click.prevent="revokeAuthZ(userAddress, chain.ica_address, [delegation.delegation.validator_address])"/>
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

export default {
  name: 'BalanceTableComponent',
  components: {ButtonComponent, CoinComponent},

  mixins: [mxChain],

  props: {
    chain: {
      type: Object,
      required: true
    }
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
    async isValidatorGranted(valAddress) {
      console.log("valAddress", valAddress)
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
      console.log(response.grants)

      return !!response.grants.length

      // grants:
      // - authorization:
      // '@type': /cosmos.staking.v1beta1.StakeAuthorization
      // allow_list:
      //   address:
      //     - cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn
      // authorization_type: AUTHORIZATION_TYPE_DELEGATE
      // max_tokens: null
      // expiration: null
      // grantee: cosmos1jat2z0ffpn7cu50zjxk4wyy89e945pczlv9jnegt5dlwhzkdeh9quhkt0a
      // granter: cosmos10h9stc5v6ntgeygf5xf945njqq5h32r53uquvw
      // - authorization:
      // '@type': /cosmos.staking.v1beta1.StakeAuthorization
      // allow_list:
      //   address:
      //     - cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn
      // authorization_type: AUTHORIZATION_TYPE_DELEGATE
      // max_tokens: null
      // expiration: null
      // grantee: cosmos1ewfm37s7navswuva6p2v50657uyxrurplm50edtgvuhwjf0ervkstmxa2x
      // granter: cosmos10h9stc5v6ntgeygf5xf945njqq5h32r53uquvw
      // pagination:
      //   next_key: null
      // total: "0"

      //return response.grants.find(grants => )


    }
  }
};
</script>