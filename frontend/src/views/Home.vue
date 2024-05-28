<template>
  <div class="landing-page">
    <header class="header">
      <img src="@/assets/logo.png" alt="Logo" class="logo" />
      <h1>InterChadz</h1>
    </header>
    <section class="description">
      <p>
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
      </p>
    </section>
    <section class="features">
      <div class="feature">
        <h2>Feature 1</h2>
        <p>
          Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus lacinia odio vitae vestibulum vestibulum.
        </p>
      </div>
      <div class="feature">
        <h2>Feature 2</h2>
        <p>
          Curabitur ac leo nunc. Vestibulum et mauris vel ante finibus maximus nec ut leo.
        </p>
      </div>
      <div class="feature">
        <h2>Feature 3</h2>
        <p>
          Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium.
        </p>
      </div>
    </section>
    <section class="team">
      <h2>Meet the Team</h2>
      <div class="team-members">
        <div class="team-member">
          <img src="@/assets/team.png" alt="Gjermund" class="team-photo" />
          <h3>Gjermund Garaba</h3>
          <div class="social-links">
            <a href="https://x.com/GjermundGaraba" target="_blank">
              <img src="@/assets/twitter.svg" alt="Twitter" class="icon" />
            </a>
            <a href="https://github.com/GjermundGaraba" target="_blank">
              <img src="@/assets/github.svg" alt="GitHub" class="icon" />
            </a>
          </div>
        </div>
        <div class="team-member">
          <img src="@/assets/team.png" alt="magiodev" class="team-photo" />
          <h3>magiodev</h3>
          <div class="social-links">
            <a href="https://x.com/magiodev" target="_blank">
              <img src="@/assets/twitter.svg" alt="Twitter" class="icon" />
            </a>
            <a href="https://github.com/magiodev" target="_blank">
              <img src="@/assets/github.svg" alt="GitHub" class="icon" />
            </a>
          </div>
        </div>
        <div class="team-member">
          <img src="@/assets/team.png" alt="arjanjohan" class="team-photo" />
          <h3>arjanjohan</h3>
          <div class="social-links">
            <a href="https://x.com/arjanjohan" target="_blank">
              <img src="@/assets/twitter.svg" alt="Twitter" class="icon" />
            </a>
            <a href="https://github.com/arjanjohan" target="_blank">
              <img src="@/assets/github.svg" alt="GitHub" class="icon" />
            </a>
          </div>
        </div>
      </div>
    </section>
    <footer class="footer">
      <p>&copy; 2024 Our Project. All rights reserved.</p>
    </footer>
  </div>
</template>

<script>
import { mapGetters } from "vuex";
import mxApp from "@/mixin/app";

export default {
  name: 'HomeView',

  mixins: [mxApp],

  computed: {
    ...mapGetters(['appConfig', 'appState'])
  }
};
</script>

<style lang="scss" scoped>
@import "@/assets/style.scss";
</style>
