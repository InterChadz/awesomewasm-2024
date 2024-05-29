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
  mixins:[mxChain, mxToast],

  components: {
    ChainStakingComponent,
    NotActiveComponent
  },
  props: {
    chainName: String,
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
      restakingEnabled: this.isActive
    };
  },
  methods: {
    async compound() {
      console.log('Compound / Restake button clicked');
      try {
        await this.autocompound();
        alert("Rewards compounded successfully.");
      } catch (error) {
        console.error("Error compounding rewards:", error);
        alert("Failed to compound rewards.");
      }
    },
    withdrawStaked() {
      console.log('Withdraw Staked Amount button clicked');
      alert("Not implemented yet.");
    },
    withdrawRewards() {
      console.log('Withdraw Rewards button clicked');
      alert("Not implemented yet.");
    },
    async handleToggle(event) {
      if (event.target.checked) {
        try {
          await this.registerUser();
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
    }
  }
};
</script>

<style scoped>
</style>
