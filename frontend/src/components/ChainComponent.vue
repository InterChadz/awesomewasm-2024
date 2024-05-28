<template>
  <div class="chain-component">
    <div class="row">
      <div class="col-md-6">
        <div class="chain-info">
          <div class="chain-header">
            <span class="badge badge-primary">{{ chainName }}</span>
            <div class="restaking-toggle">
              <span class="toggle-label">{{ restakingEnabled ? 'Restaking Enabled ' : 'Enable Restaking ' }}</span>

              <label class="switch">
                <input type="checkbox" v-model="restakingEnabled" @change="toggleRestaking">
                <span class="slider round"></span>
              </label>
            </div>
          </div>
          <h4>Cost to Autocompound/User</h4>
          <p>{{ costToAutocompound }}</p>
          <h4>Last Autocompound Executed</h4>
          <p>{{ lastAutocompound }}</p>
        </div>
      </div>
      <div class="col-md-6">
        <div class="user-info">
          <BalanceTableComponent :stakedValidators="stakedValidators" :pendingRewards="pendingRewards" />
          <div class="action-buttons">
            <button @click="compound">Compound / Restake</button>
            <button @click="withdrawStaked">Withdraw Staked Amount</button>
            <button @click="withdrawRewards">Withdraw Rewards</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import BalanceTableComponent from '@/components/BalanceTableComponent.vue';

export default {
  name: 'ChainComponent',
  components: {
    BalanceTableComponent
  },
  props: {
    chainName: String,
    costToAutocompound: String,
    lastAutocompound: String,
    stakedValidators: Array,
    pendingRewards: Number
  },
  data() {
    return {
      showMore: false,
      restakingEnabled: false
    };
  },
  methods: {
    compound() {
      // Compound or restake logic here
      console.log('Compound / Restake button clicked');
    },
    withdrawStaked() {
      // Withdraw staked amount logic here
      console.log('Withdraw Staked Amount button clicked');
    },
    withdrawRewards() {
      // Withdraw rewards logic here
      console.log('Withdraw Rewards button clicked');
    },
    toggleRestaking() {
      // Toggle restaking logic here
      console.log('Toggle Restaking:', this.restakingEnabled);
    }
  }
};
</script>
