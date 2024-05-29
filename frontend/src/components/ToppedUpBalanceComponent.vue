<template>
    <div class="balance-component">
      
      <p><b>Active Balance </b>{{ balance }} NTRN</p>
      <button @click="addBalance">Add</button>
      
    </div>
  </template>
  
  <script>
  export default {
    name: 'ToppedUpBalanceComponent',
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
  