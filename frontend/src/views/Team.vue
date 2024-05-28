<template>
    <div class="team-page">
      <TeamComponent />
    </div>
  </template>
  
  <script>
  import TeamComponent from '@/components/TeamComponent.vue';
  
  export default {
    name: 'TeamPage',
    components: {
      TeamComponent
    }
  };
  </script>
  
  
<style lang="scss" scoped>
@import "@/assets/style.scss";
</style>
