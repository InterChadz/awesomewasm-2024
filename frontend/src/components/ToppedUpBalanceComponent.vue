<template>
    <div class="balance-component">
      <h4>Active Balance</h4>
      <p>{{ balance }} NTRN</p>
      <button @click="addBalance">Add Balance</button>
    </div>
  </template>
  
  <script>
  export default {
    name: 'ToppedUpBalanceComponent',
    props: {
      title: String,
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
  