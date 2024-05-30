<template>
  <div class="balance-component">
    <p><b>Active Balance</b> <span class="balance-amount">{{ displayAmount(userContractBalance, 2) }} <CoinComponent/></span></p>

    <button @click.prevent="addBalance">Add</button>

    <div v-if="showPopup" class="popup">
      <div class="popup-content">
        <h3>Select Amount to Top Up</h3>

        <input class="form-control" type="number" v-model="topupAmount" placeholder="Enter $NTRN amount"/>

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
import {mapActions, mapGetters} from "vuex";

export default {
  name: 'ToppedUpBalanceComponent',

  mixins: [mxChain, mxToast],

  components: {
    CoinComponent
  },

  computed: {
    ...mapGetters(['userContractBalance']),
  },

  data() {
    return {
      showPopup: false,
      topupAmount: 0
    };
  },

  methods: {
    ...mapActions(['fetchUserData']),

    addBalance() {
      this.showPopup = true;
    },
    async confirmTopup() {
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
        await this.fetchUserData()
        // hide the popup only if we were successful and reset the next amount to 0
        this.topupAmount = 0;
        this.showPopup = false;
      } catch (error) {
        this.toast.error("Failed to top up balance.");
      }
    }
  }
};
</script>

<style scoped>
</style>
