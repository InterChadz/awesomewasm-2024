<template>
  <div class="chain-component">
    <div class="row">
      <div class="col-md-6">
        <div class="chain-header">
          <span class="badge badge-primary">
            <img :src="`/chains/${chain.chain_id}.png`" :alt="Icon" class="chain-image">{{ chain.chain_id }}
          </span>

          <div class="restaking-toggle">
            <span class="toggle-label">User registered: </span>
            <label class="switch">
              <input type="checkbox" :checked="isUserRegistered" :disabled="isUserRegistered"
                     @change.prevent="onChangeSwitch">
              <span class="slider round"></span>
            </label>
          </div>
        </div>

        <div class="chain-info">
          <ul class="list-unstyled">
            <li>Autocompound Fee: {{ costToAutocompound }}</li>
            <li>Last (auto)compound: {{ costToAutocompound }}</li>
          </ul>
        </div>

        <div class="action-buttons p-3">
          <button @click="compound">Compound</button>
          <!-- <button @click="withdrawStaked" disabled="true">Withdraw All</button>
          <button @click="withdrawRewards" disabled="true">Withdraw Rewards</button> -->
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

export default {
  name: 'ChainComponent',

  mixins: [mxChain, mxToast],

  components: {
    ChainStakingComponent,
  },

  props: {
    chain: {
      type: Object,
      required: true
    }
  },

  computed: {
    ...mapGetters(['userRegistrations', 'userAddress', 'userDelegations']),

    isUserRegistered() {
      return this.userRegistrations.some(registration => registration.chain_id === this.chain.chain_id);
    }
  },

  methods: {
    ...mapActions(['fetchUserData']),

    async onChangeSwitch() {
      try {
        await this.registerUser(this.chainId, this.userAddress, this.userDelegations);
        this.toast.success("User registered successfully.");
        await this.fetchUserData()
      } catch (error) {
        this.toast.error("Failed to register user.");
      }
    },

    async compound() {
      try {
        await this.autocompound();
        this.toast.success("Rewards compounded successfully.");
      } catch (error) {
        this.toast.error("Failed to compound rewards.");
      }
    }
  }
};
</script>

<style scoped>
</style>
