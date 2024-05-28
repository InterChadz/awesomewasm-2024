<template>
  <a :href="`${baseUrl}/accounts/${address}`" target="_blank">{{ address.substring(0, cut) }}</a>
</template>

<script>
export default {
  name: "UserAddressComponent",

  props: {
    address: {
      type: String,
      required: true,
    },
    cut: {
      type: Number,
      required: true
    }
  },

  data() {
    return {
      baseUrl: process.env.VUE_APP_EXPLORER_BASE_URL
    }
  }
}
</script>