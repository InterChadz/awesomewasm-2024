import {mapActions, mapGetters} from "vuex";

const mxApp = {
  data() {
    return {
      currentTime: new Date().getTime(),
    };
  },

  computed: {
    ...mapGetters(['userAddress']),
  },

  methods: {
    ...mapActions([
      // User
      'initUser',
      'fetchUserData',
      'fetchUserDelegations',
      // App
      'fetchAppConfig',
      'fetchAppSupportedChains'
    ]),

    async fetchOnce() {
      // Important as first
      await this.initUser();

      // App-wise things
      await this.fetchAppConfig();
      await this.fetchAppSupportedChains();
    },

    async fetchInterval() {
      if (this.userAddress) {
        await this.fetchUserData();
        await this.fetchUserDelegations()
      }
    },

    updateCurrentTime() {
      this.currentTime = new Date().getTime();
    },
  },

  created() {
    this.intervalId = setInterval(this.updateCurrentTime, 1000);
  },

  unmounted() {
    clearInterval(this.intervalId);
  },
};

export default mxApp;
