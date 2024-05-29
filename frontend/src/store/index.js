import {createStore} from "vuex";
import {AminoTypes} from "@cosmjs/stargate";
import {CosmWasmClient, SigningCosmWasmClient} from "@cosmjs/cosmwasm-stargate";
import {Registry} from "@cosmjs/proto-signing";
import {cosmosAminoConverters, cosmosProtoRegistry, cosmwasmAminoConverters, cosmwasmProtoRegistry} from "osmojs";
import mxChain from "../mixin/chain";
import axios from "axios";

const mxChainUtils = {
  methods: mxChain.methods
};

const chainsApis = JSON.parse(process.env.VUE_APP_CHAINS_APIS)

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
      contractBalance: null,
      registrations: [],
      rewards: null,
      delegations: []
    },

    app: {
      config: null,
      supportedChains: [],
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

    userRegistrations(state) {
      return state.user.registrations;
    },

    userRewards(state) {
      return state.user.rewards;
    },

    userBalance(state) {
      return state.user.balance;
    },

    userContractBalance(state) {
      return state.user.contractBalance;
    },

    userDelegations(state) {
      return state.user.delegations;
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

    setUserContractBalance(state, contractBalance) {
      state.user.contractBalance = contractBalance;
    },

    setUserRegistrations(state, registrations) {
      state.user.registrations = registrations;
    },

    setUserRewards(state, rewards) {
      state.user.rewards = rewards;
    },

    setUserDelegations(state, delegations) {
      state.user.delegations = delegations;
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

      // #[returns(GetUserRegistrationsResponse)]
      // UserRegistrations {
      //     address: String,
      //     limit: Option<u64>,
      //     start_after: Option<String>,
      // },
      const contractBalance = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {
          user_balance: {
            address: state.user.address
          }
        }
      );
      commit("setUserContractBalance", contractBalance.balance);

      // #[returns(GetUserRegistrationsResponse)]
      // UserRegistrations {
      //     address: String,
      //     limit: Option<u64>,
      //     start_after: Option<String>,
      // },
      const registrations = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {
          user_registrations: {
            address: state.user.address
          }
        }
      );
      commit("setUserRegistrations", registrations.user_chain_registrations);

      // Start iterating registrations to get rewards and allocated amount
      let userRewards = [];
      for (const registration of registrations.user_chain_registrations) {
        // #[returns(GetCalculatedRewardResponse)]
        // CalculateReward {
        //     address: String,
        //     chain_id: String,
        //     remote_address: String,
        // },
        const calculateReward = await state.user.querier.queryContractSmart(
          process.env.VUE_APP_CONTRACT,
          {
            calculate_reward: {
              address: state.user.address,
              chain_id: registration.chain_id,
              remote_address: registration.remote_address
            }
          }
        );
        userRewards.push({
          chain_id: registration.chain_id,
          calculated_reward: calculateReward
        });
      }
      commit("setUserRewards", userRewards);
    },

    async fetchUserDelegations({state, commit}) {
      if (!state.user.address || !state.user.querier) {
        console.error("Address or Querier is not initialized");
        return;
      }

      let delegations = []
      for (const chain of state.app.supportedChains) {
        const userRegistration = state.user.registrations.find(r => r.chain_id === chain.chain_id);

        const chainApi = chainsApis.find(c => c.chainId === chain.chain_id);
        const response = await axios.get(`${chainApi.rest}/${userRegistration.remote_address}`)
        if (!chain) {
          continue
        }
        delegations.push({
          chain_id: chain.chain_id,
          delegations: response.data.delegation_responses
        })
      }

      commit("setUserDelegations", delegations);
    },

    // #[returns(ConfigResponse)]
    // Config {},
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

    // #[returns(SupportedChainsResponse)]
    // SupportedChains {
    //     limit: Option<u64>,
    //     start_after: Option<String>,
    // },
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
  },

  modules: {},
});
