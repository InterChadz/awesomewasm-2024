<template>
    <div class="balance-component">
      <h4>{{ title }}</h4>
      <p>{{ balance }} NTRN</p>
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
  