import {createStore} from "vuex";
import {AminoTypes} from "@cosmjs/stargate";
import {CosmWasmClient, SigningCosmWasmClient} from "@cosmjs/cosmwasm-stargate";
import {Registry} from "@cosmjs/proto-signing";
import {cosmosAminoConverters, cosmosProtoRegistry, cosmwasmAminoConverters, cosmwasmProtoRegistry} from "osmojs";
import mxChain from "../mixin/chain";

const mxChainUtils = {
  methods: mxChain.methods
};

export default createStore({
  /**
   * State containing primary Keys of the Vue store. Persisting of state objects.
   */
  state: {
    user: {
      signer: null,
      querier: null,
      address: null,
      balance: null,
    },

    app: {
      config: null,
      supportedChains: [],
      userRegistrations: [],
      calculateReward: null
    }
  },

  getters: {
    userSigner(state) {
      return state.user.signer;
    },

    userQuerier(state) {
      return state.user.querier;
    },

    userAddress(state) {
      return state.user.address;
    },

    userBalance(state) {
      return state.user.balance;
    },

    appConfig(state) {
      return state.app.config;
    },

    appSupportedChains(state) {
      return state.app.supportedChains
    }
  },

  mutations: {
    setUserSigner(state, signer) {
      state.user.signer = signer;
    },

    setUserQuerier(state, querier) {
      state.user.querier = querier;
    },

    setUserAddress(state, address) {
      state.user.address = address;
    },

    setUserBalance(state, balance) {
      state.user.balance = balance;
    },

    // App

    setAppConfig(state, appConfig) {
      state.app.config = appConfig;
    },

    setAppSupportedChains(state, appConfig) {
      state.app.supportedChains = appConfig;
    },
  },

  actions: {
    async initUser({commit}) {
      const chainId = process.env.VUE_APP_CHAIN_ID;

      if (!window.keplr) {
        alert("Please install Keplr extension.");
      } else {
        await window.keplr.enable(chainId);

        const offlineSigner = window.getOfflineSigner(chainId);
        const accounts = await offlineSigner.getAccounts();
        commit("setUserAddress", accounts[0].address);

        const protoRegistry = [
          ...cosmosProtoRegistry,
          ...cosmwasmProtoRegistry,
        ];
        const aminoConverters = {
          ...cosmosAminoConverters,
          ...cosmwasmAminoConverters,
        };
        const registry = new Registry(protoRegistry);
        const aminoTypes = new AminoTypes(aminoConverters);

        const signingClient = await SigningCosmWasmClient.connectWithSigner(
          process.env.VUE_APP_RPC,
          offlineSigner,
          // other options
          {
            registry,
            aminoTypes
          }
        );
        commit("setUserSigner", signingClient);
      }

      // Initialize CosmWasmClient for querying
      const queryClient = await CosmWasmClient.connect(process.env.VUE_APP_RPC);
      commit("setUserQuerier", queryClient);
    },

    async fetchUserData({state, commit}) {
      if (!state.user.address || !state.user.querier) {
        console.error("Address or Querier is not initialized");
        return;
      }

      // Balance
      const balance = await state.user.querier.queryClient.bank.balance(
        state.user.address,
        process.env.VUE_APP_FEE_DENOM
      );
      commit("setUserBalance", mxChainUtils.methods.displayAmount(Number(balance.amount)));
    },

    async fetchAppConfig({state, commit}) {
      if (!state.user.querier) {
        console.error("Querier is not initialized");
        return;
      }

      // Use CosmWasmClient for the query
      const data = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {config: {}}
      );
      commit("setAppConfig", data.config);
    },

    async fetchAppSupportedChains({state, commit}) {
      if (!state.user.querier) {
        console.error("Querier is not initialized");
        return;
      }

      // Use CosmWasmClient for the query
      const data = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {supported_chains: {}}
      );
      commit("setAppSupportedChains", data.chains);
    },

    async fetchAppUserRegistrations({state, commit}) {
      if (!state.user.querier) {
        console.error("Querier is not initialized");
        return;
      }

      // Use CosmWasmClient for the query
      const data = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {user_registrations: {}}
      );
      commit("setAppUserRegistrations", data.registrations);
    },

    async fetchAppCalculateReward({state, commit}) {
      if (!state.user.querier) {
        console.error("Querier is not initialized");
        return;
      }

      // Use CosmWasmClient for the query
      const data = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {calculate_reward: {}}
      );
      commit("setAppCalculateReward", data.reward);
    }
  },

  modules: {},
});
