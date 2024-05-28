<template>
    <div class="balance-component">
      <p><b>Wallet Balance: </b>{{ balance }} NTRN</p>
    </div>
  </template>
  
  <script>
  export default {
    name: 'WalletBalanceComponent',
    props: {
      balance: Number
    }
  };
  </script>
  