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
      'fetchAppState',
      'fetchUserData',
    ]),

    // TODO: fetchUser()

    async fetchOnce() {
      await this.initUser();

      await this.fetchAppConfig();
      await this.fetchAppState();

      // Init signer and querier
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
