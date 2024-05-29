<template>
  <div class="chain-component">
    <div class="row">
      <div class="col-md-6">
        <div class="chain-header">
          <span class="badge badge-primary">
            <img :src="chainImage" alt="Chain Image" class="chain-image">{{ chainName }}
          </span>
          <div class="restaking-toggle">
            <span class="toggle-label">Restaking Enabled: </span>
            <label class="switch">
              <input type="checkbox" :checked="restakingEnabled" :disabled="isActive" @change="handleToggle">
              <span class="slider round"></span>
            </label>
          </div>
        </div>
        <div class="chain-info">
          <h5>Cost to compound</h5>
          <p>{{ costToAutocompound }}</p>
          <h5>Last (auto)compound executed</h5>
          <p>{{ lastAutocompound }}</p>
        </div>
      </div>
      <div class="col-md-6">
        <ChainStakingComponent v-if="isActive" :stakedValidators="stakedValidators" :pendingRewards="pendingRewards" :totalStaked="totalStaked" />
        <NotActiveComponent v-else />
      </div>
    </div>
  </div>
</template>

<script>
import ChainStakingComponent from '@/components/ChainStakingComponent.vue';
import NotActiveComponent from '@/components/NotActiveComponent.vue';
import mxChain from '@/mixin/chain';
import mxToast from "@/mixin/toast";

export default {
  name: 'ChainComponent',
  mixins: [mxChain, mxToast],
  components: {
    ChainStakingComponent,
    NotActiveComponent
  },
  props: {
    chainName: String,
    chainId: String,
    chainImage: String,
    costToAutocompound: String,
    lastAutocompound: String,
    stakedValidators: Array,
    totalStaked: Number,
    pendingRewards: Number,
    isActive: Boolean
  },
  data() {
    return {
      restakingEnabled: this.isActive,
      chains: JSON.parse(process.env.VUE_APP_CHAINS_APIS)
    };
  },
  methods: {
    async handleToggle(event) {
      if (!this.chainId || !this.userAddress) {
        console.error("Required variables are missing.");
        event.target.checked = false;
        return;
      }
      let validators = await this.fetchDelegations(this.chainId, this.userAddress); // TODO Get validators from the chain
      if (event.target.checked) {
        try {
          await this.registerUser(this.chainId, this.userAddress, validators);
          this.toast.success("User registered successfully.");
          this.restakingEnabled = true;
        } catch (error) {
          console.error("Error registering user:", error);
          this.toast.error("Failed to register user.");
          this.restakingEnabled = false;
          event.target.checked = false;
        }
      } else {
        // Prevent switching off if already active
        event.target.checked = true;
      }
    },
    async fetchDelegations(chainId, userAddress) {
      const chain = this.chains.rest.find(chain => chain.chainId === chainId);
      if (!chain) {
        console.error(`Chain with id ${chainId} not found.`);
        return [];
      }

      try {
        const response = await fetch(`${chain.url}/${userAddress}`);
        const data = await response.json();
        console.log(data); // Log the data or handle it as needed
        return data.delegation_responses.map(response => response.delegation.delegator_address);
      } catch (error) {
        console.error("Error fetching delegations:", error);
        return [];
      }
    },
    async getDelegatorAddresses(chainId, userAddress) {
      const delegatorAddresses = await this.fetchDelegations(chainId, userAddress);
      console.log(delegatorAddresses); // Log the addresses or handle them as needed
      return delegatorAddresses;
    }
  }
};
</script>

<style scoped>
</style>
