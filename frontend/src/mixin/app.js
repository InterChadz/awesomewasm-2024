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
      //'fetchAppSupportedChains',
      'fetchDueUserChainRegistrations'
    ]),

    async fetchOnce() {
      try {
        // Important as first
        await this.initUser();

        // App-wise things
        await this.fetchAppConfig();
      } catch (e) {
        // nothing
        console.error(e)
      }
    },

    async fetchInterval() {
      try {
        await this.fetchDueUserChainRegistrations()

        if (this.userAddress) {
          await this.fetchUserData();
          await this.fetchUserDelegations()
        }
      } catch (e) {
        // nothing
        console.error(e)
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
