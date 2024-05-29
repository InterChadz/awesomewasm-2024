<template>
  <div class="chain-staking-component">
      <div class="user-info">
        <BalanceTableComponent :stakedValidators="stakedValidators" :pendingRewards="pendingRewards" :totalStaked="totalStaked" />
        <div class="action-buttons">
          <button @click="compound">Compound</button>
          <!-- <button @click="withdrawStaked" disabled="true">Withdraw All</button>
          <button @click="withdrawRewards" disabled="true">Withdraw Rewards</button> -->
        </div>
    </div>
  </div>
</template>

<script>
import BalanceTableComponent from '@/components/BalanceTableComponent.vue';
import mxChain from "@/mixin/chain";
import mxToast from '@/mixin/toast';

export default {
  name: 'ChainComponent',
  components: {
    BalanceTableComponent
  },
  props: {
    chainName: String,
    chainImage: String,
    costToAutocompound: String,
    lastAutocompound: String,
    stakedValidators: Array,
    totalStaked: Number,
    pendingRewards: Number
  },
  mixins: [mxChain, mxToast],

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
        this.toast.success("Rewards compounded successfully.");
      } catch (error) {
        console.error("Error compounding rewards:", error);
        this.toast.error("Failed to compound rewards.");
      }
    },
    withdrawStaked() {
      // Withdraw staked amount logic here
      console.log('Withdraw Staked Amount button clicked');
    },
    withdrawRewards() {
      // Withdraw rewards logic here
      console.log('Withdraw Rewards button clicked');

    }
  }
};
</script>
