<template>
    <div class="balance-component">
      <h3>{{ title }}</h3>
      <p>{{ balance }}</p>
    </div>
  </template>
  
  <script>
  export default {
    name: 'WalletBalanceComponent',
    props: {
      title: String,
      balance: Number
    }
  };
  </script>
  