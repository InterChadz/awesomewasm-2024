import {mapActions, mapGetters} from "vuex";

const mxApp = {
  data() {
    return {
      currentTime: new Date().getTime(),
    };
  },

  computed: {
    ...mapGetters(['userAddress', 'appState']),
  },

  methods: {
    ...mapActions([
      'initUser',
      'fetchAppConfig',
      'fetchUserData',
      'fetchAppSupportedChains'
    ]),

    async fetchOnce() {
      await this.initUser();

      await this.fetchAppConfig();

      await this.fetchAppSupportedChains();

      // Init signer and querier for the connected user, if any
      if (this.userAddress) {
        await this.fetchUserData();
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
