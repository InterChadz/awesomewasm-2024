<template>
  <div class="chain-component">
    <div class="row">
      <div class="col-md-6">
        <div class="chain-header">
          <span class="badge badge-primary">
            <img :src="`/chains/${chain.chain_id}.png`" :alt="Icon" class="chain-image">{{ chain.chain_id }}
          </span>

          <div class="restaking-toggle" v-if="!isUserRegistered && userDelegations.find(d => d.chain_id === chain.chain_id).delegations.length">
            <span class="toggle-label">User registered: </span>
            <label class="switch">
              <input type="checkbox" :checked="isUserRegistered" :disabled="isUserRegistered"
                     @change.prevent="onChangeSwitch">
              <span class="slider round"></span>
            </label>
          </div>
          <p v-else-if="!userDelegations.find(d => d.chain_id === chain.chain_id).delegations.length">You have no delegations, so you cant register your account.</p>
          <p v-else>You have registered successfully. You can grant the permission now via AuthZ module to the ICA account.</p>
        </div>

        <div class="chain-info">
          <ul class="list-unstyled">
            <li>Autocompound Fee: {{ displayAmount(chain.autocompound_cost, 2) }}
              <CoinComponent/>
            </li>
          </ul>
        </div>
      </div>
      <div class="col-md-6">
        <ChainStakingComponent :chain="chain"/>
      </div>
    </div>
  </div>
</template>

<script>
import ChainStakingComponent from '@/components/ChainStakingComponent.vue';
import mxChain from '@/mixin/chain';
import mxToast from "@/mixin/toast";
import {mapActions, mapGetters} from "vuex";
import CoinComponent from "@/components/Common/CoinComponent.vue";

export default {
  name: 'ChainComponent',

  mixins: [mxChain, mxToast],

  components: {
    CoinComponent,
    ChainStakingComponent,
  },

  props: {
    chain: {
      type: Object,
      required: true
    }
  },

  computed: {
    ...mapGetters(['userRegistrations', 'userAddress', 'userDelegations', 'appDueUserRegistrations']),

    isUserRegistered() {
      console.log(this.userRegistrations.some(registration => registration.chain_id === this.chain.chain_id))
      return this.userRegistrations.some(registration => registration.chain_id === this.chain.chain_id);
    }
  },

  methods: {
    ...mapActions(['fetchUserData']),

    async onChangeSwitch() {
      try {
        await this.registerUser(this.chain.chain_id, this.userAddress, this.getValidatorsFromDelegations(this.userDelegations));
        this.toast.success("User registered successfully.");
        await this.fetchUserData()
      } catch (error) {
        console.error("failed to reg", error)
        this.toast.error("Failed to register user.");
      }
    }
  }
};
</script>

<style scoped>
</style>
