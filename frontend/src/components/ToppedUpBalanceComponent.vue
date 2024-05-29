<template>
  <div class="balance-component">
    <p><b>Active Balance</b> <span class="balance-amount">{{ displayAmount(balance) }} <CoinComponent /></span> </p>    <button @click="addBalance">Add</button>

    <div v-if="showPopup" class="popup">
      <div class="popup-content">
        <h3>Select Amount to Top Up</h3>
        <input type="number" v-model="topupAmount" placeholder="Enter amount" />
        <button @click="confirmTopup">Confirm</button>
        <button @click="showPopup = false">Cancel</button>
      </div>
    </div>
  </div>
</template>

<script>
import CoinComponent from "@/components/Common/CoinComponent.vue";
import mxChain from "@/mixin/chain";
import mxToast from "@/mixin/toast";

export default {
  name: 'ToppedUpBalanceComponent',
  components: {
    CoinComponent
  },
  props: {
    balance: Number
  },
  mixins: [mxChain, mxToast],
  data() {
    return {
      showPopup: false,
      topupAmount: 0
    };
  },
  methods: {
    addBalance() {
      this.showPopup = true;
    },
    async confirmTopup() {
      this.showPopup = false;

      if (this.topupAmount <= 0) {
        alert("Please enter a valid amount.");
        return;
      }

      // Convert the amount to the format required by the contract
      const funds = [
        {
          denom: process.env.VUE_APP_FEE_DENOM,
          amount: (this.topupAmount * 1000000).toString()
        }
      ];

      try {
        await this.topupUserBalance(funds);
        this.toast.success("Balance topped up successfully.");
      } catch (error) {
        console.error("Error topping up balance:", error);
        this.toast.error("Failed to top up balance.");
      
      }
    }
  }
};
</script>

<style scoped>
</style>
