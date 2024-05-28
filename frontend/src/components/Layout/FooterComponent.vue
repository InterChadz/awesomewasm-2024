<template>
  <div class="footer container-fluid">
    <footer class="row align-items-center py-3">
      <div class="col-md-6 text-md-start text-center mb-2 mb-md-0">
        © {{ new Date().getFullYear() }} {{ title }} | All rights reserved.
      </div>
      <div class="col-md-6">
        <ul class="nav list-unstyled d-flex justify-content-center justify-content-md-end">
          <li class="ms-3">
            <a href="https://github.com/InterChadz/awesomewasm-2024" target="_blank" rel="noopener noreferrer">
              <img src="@/assets/github.svg" alt="GitHub" class="icon">
            </a>
          </li>
          <li class="ms-3">
            <a href="https://twitter.com/InterChadz" target="_blank" rel="noopener noreferrer">
              <img src="@/assets/twitter.svg" alt="Twitter" class="icon">
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
      title: "InterChadz"
    };
  }
};
</script>

<style scoped lang="scss">
@import "@/assets/style.scss";
</style>
