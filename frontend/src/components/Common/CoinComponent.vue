<template>
  <img :src="imageCoin" class="coin-component" alt="Coin"/>
</template>

<script>
import imageCoin from "@/assets/neutron.png"

export default {
  name: "CoinComponent",

  data() {
    return {
      imageCoin
    }
  }
}
</script>

<style scoped lang="scss">
.coin-component {
  border-radius: 50%;
  width: 1.25em;
  height: auto;
  vertical-align: sub;
}
</style>