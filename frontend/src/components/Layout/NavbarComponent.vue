<template>
  <nav class="navbar navbar-expand-lg p-0">
    <div class="container-fluid">
      <router-link class="navbar-brand" to="/">
        <img class="logo" :src="imageLogo" :alt="title" />
      </router-link>

      <button
        class="navbar-toggler"
        type="button"
        data-bs-toggle="collapse"
        data-bs-target="#navbarNavAltMarkup"
        aria-controls="navbarNavAltMarkup"
        aria-expanded="false"
        aria-label="Toggle navigation"
      >
        <span class="navbar-toggler-icon"></span>
      </button>

      <div class="collapse navbar-collapse" id="navbarNavAltMarkup">
        <div class="navbar-nav ms-auto text-end text-lg-center">
          <router-link to="/" class="nav-link py-lg-4 px-lg-5"
            >Home</router-link
          >
          <router-link to="/" class="nav-link py-lg-4 px-lg-5"
            >Staking</router-link
          >
          <router-link to="/" class="nav-link py-lg-4 px-lg-5"
            >Team</router-link
          >

          <div class="nav-link py-lg-4 px-lg-5">
            <WalletComponent />
          </div>
        </div>
      </div>
    </div>
  </nav>
</template>

<script>
import imageLogo from "@/assets/logo.png";
import WalletComponent from "@/components/Common/WalletComponent.vue";

export default {
  name: "NavbarComponent",
  components: { WalletComponent },

  data() {
    return {
      imageLogo,
      title: "Restaker",
    };
  },
};
</script>

<style scoped lang="scss">
@import "@/assets/style.scss";

.navbar {
  .container-fluid {
    .logo {
      height: 80px;
    }

    .navbar-nav {
      text-transform: uppercase;
    }
  }
}
</style>
