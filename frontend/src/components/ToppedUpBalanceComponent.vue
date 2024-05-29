<template>
    <div class="balance-component">
      
      <p><b>Active Balance </b>{{ balance }} <CoinComponent/></p>
      <button @click="addBalance">Add</button>
      
    </div>
  </template>
  
  <script>
  import CoinComponent from "@/components/Common/CoinComponent.vue";

  export default {
    name: 'ToppedUpBalanceComponent',
    components: {
    CoinComponent
  },
    props: {
      balance: Number
    },
    methods: {
      addBalance() {
        // Logic to add balance
        console.log('Add Balance button clicked');
      }
    }
  };
  </script>
  