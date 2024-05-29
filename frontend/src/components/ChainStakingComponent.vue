<template>
  <div class="chain-staking-component" v-if="chain.ica_address">
      <div class="user-info">
        <BalanceTableComponent :chain="chain" />
    </div>
  </div>
  <p v-else>Chain is not active yet.</p>
</template>

<script>
import BalanceTableComponent from '@/components/BalanceTableComponent.vue';

export default {
  name: 'ChainComponent',

  components: {
    BalanceTableComponent
  },

  props: {
    chain: {
      type: Object,
      required: true
    }
  },
}
</script>
