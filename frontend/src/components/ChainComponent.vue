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
                <input type="checkbox" v-model="restakingEnabled" @change="toggleRestaking">
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

export default {
  name: 'ChainComponent',
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
      restakingEnabled: false
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
    toggleRestaking() {
      console.log('Toggle Restaking:', this.restakingEnabled);
    }
  }
};
</script>

<style scoped>
</style>
