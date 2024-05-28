<template>
    <div class="balance-component">
      <h3>{{ title }}</h3>
      <p>{{ balance }}</p>
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
  