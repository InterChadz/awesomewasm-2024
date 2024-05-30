<template>
  <div id="app-body" class="d-flex flex-column">
    <LoadingComponent v-if="isBusy" />

    <template v-else>
      <NavbarComponent />

      <div class="main-section flex-grow-1">
        <router-view />
      </div>

      <FooterComponent />
    </template>
  </div>
</template>

<script>
import NavbarComponent from "@/components/Layout/NavbarComponent.vue";
import LoadingComponent from "@/components/Common/LoadingComponent.vue";
import FooterComponent from "@/components/Layout/FooterComponent.vue";
import mxApp from "@/mixin/app";
import { mapGetters } from "vuex";
import mxChain from "@/mixin/chain";

export default {
  name: "App",

  mixins: [mxApp, mxChain],

  components: { FooterComponent, LoadingComponent, NavbarComponent },

  computed: {
    ...mapGetters(["appConfig"]),
  },

  data() {
    return {
      isBusy: true,
      intervalTimeout: Number(process.env.VUE_APP_INTERVAL_TIMEOUT),
    };
  },

  async created() {
    try {
      await this.suggestChain(JSON.parse(process.env.VUE_APP_CHAIN_INFO));
    } catch (e) {
      console.error(e)
    }

    console.log("suggesting cosmos hub")
    await window.keplr.experimentalSuggestChain({
      "chainId": "testy-2",
      "chainName": "Cosmos Testy Hub",
      "rpc": "http://localhost:16657",
      "rest": "http://localhost:1316",
      "bip44": {"coinType": 118},
      "bech32Config": {
        "bech32PrefixAccAddr": "cosmos",
        "bech32PrefixAccPub": "cosmospub",
        "bech32PrefixValAddr": "cosmosvaloper",
        "bech32PrefixValPub": "cosmosvaloperpub",
        "bech32PrefixConsAddr": "cosmosvalcons",
        "bech32PrefixConsPub": "cosmosvalconspub"
      },
      "currencies": [{"coinDenom": "ATOM", "coinMinimalDenom": "uatom", "coinDecimals": 6, "coinGeckoId": "cosmos"}],
      "feeCurrencies": [{
        "coinDenom": "ATOM",
        "coinMinimalDenom": "uatom",
        "coinDecimals": 6,
        "coinGeckoId": "cosmos",
        "gasPriceStep": {"low": 0.0025, "average": 0.025, "high": 0.04}
      }],
      "stakeCurrency": {"coinDenom": "ATOM", "coinMinimalDenom": "uatom", "coinDecimals": 6, "coinGeckoId": "cosmos"}
    });
    await window.keplr.enable("testy-2");

    await this.fetchOnce();
    await this.fetchInterval()
    // we ensure that till this moment rest of UI is kept idle
    this.isBusy = false;

    // Set auto-fetch interval
    this.intervalId = setInterval(() => {
      this.fetchInterval();
    }, this.intervalTimeout);
  },

  unmounted() {
    if (this.intervalId) clearInterval(this.intervalId);
  },
};
</script>

<style lang="scss">
@import "@/assets/style.scss";
</style>
