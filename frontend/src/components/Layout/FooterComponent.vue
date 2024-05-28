<template>
  <div class="footer container-fluid">
    <footer class="row align-items-center py-3 text-white">
      <div class="col-md-4 text-md-start text-center mb-2 mb-md-0">
        © {{ new Date().getFullYear() }} {{title}}
      </div>

      <div class="col-md-4 text-center d-none d-md-block">
        <img class="logo" src="/favicon.ico" :alt="title">
      </div>

      <div class="col-md-4">
        <ul class="nav list-unstyled d-flex justify-content-center justify-content-md-end">
          <li class="ms-3">
            <a href="https://github.com/InterChadz/awesomewasm-2024" target="_blank" rel="noopener noreferrer">
              <b-icon-github class="text-white"></b-icon-github>
            </a>
          </li>
        </ul>
      </div>
    </footer>
  </div>
</template>

<script>
export default {
  name: "FooterComponent",

  data() {
    return {
      title: "Restaker"
    }
  }
}
</script>

<style scoped lang="scss">
@import "@/assets/style.scss";

.footer {
  .logo {
    height: 32px;
  }
}
</style>