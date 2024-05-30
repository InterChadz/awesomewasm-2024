<template>
  <button class="button-component" :class="isSmall ? 'small' : ''" :disabled="isDisabled">
    {{ text }}
    <LoadingComponent v-if="isBusy" class="d-inline"/>
  </button>
</template>

<script>
import LoadingComponent from "@/components/Common/LoadingComponent.vue";

export default {
  name: "ButtonComponent",
  components: {LoadingComponent},
  props: {
    text: {
      type: String,
      required: true,
    },
    isDisabled: {
      type: Boolean,
      default: false,
    },
    isBusy: {
      type: Boolean,
      default: false,
    },
    isSmall: {
      type: Boolean,
      default: false,
    }
  },
}
</script>

<style lang="scss" scoped>
@import '@/assets/style';

.button-component {
  outline: none;
  vertical-align: middle;
  font-weight: 600;
  text-transform: uppercase;
  padding: .5em 1em;
  border: 0;
  font-size: 1.5em;

  &.small {
    font-size: 1em;
  }

  &:disabled {
    cursor: not-allowed;
  }

  img {
    width: 32px;
  }
}
</style>
