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
      pubkey: null,
      balance: null,
      contractBalance: null,
      registrations: [],
      rewards: null,
      delegations: []
    },

    app: {
      config: null,
      supportedChains: [],
      dueUserChainRegistrations: []
    }
  },

  getters: {
    // main origin chain signer
    userSigner(state) {
      return state.user.signer;
    },

    userQuerier(state) {
      return state.user.querier;
    },

    userAddress(state) {
      return state.user.address;
    },

    userPubKey(state) {
      return state.user.pubKey;
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
    },

    appDueUserRegistrations(state) {
      return state.app.dueUserChainRegistrations;
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

    setUserPubKey(state, pubkey) {
      state.user.pubkey = pubkey;
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

    setAppDueUserChainRegistrations(state, dueUserChainRegistrations) {
      state.app.dueUserChainRegistrations = dueUserChainRegistrations;
    }
  },

  actions: {
    async initUser({commit}) {
      // Initialize CosmWasmClient for querying
      const queryClient = await CosmWasmClient.connect(process.env.VUE_APP_RPC);
      commit("setUserQuerier", queryClient);

      // fetch app supported chains
      const data = await queryClient.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {supported_chains: {}}
      );
      commit("setAppSupportedChains", data.chains);

      const chainId = process.env.VUE_APP_CHAIN_ID;

      if (!window.keplr) {
        alert("Please install Keplr extension.");
      } else {
        // populating chainIds with the origin chain plus the destination chains supported by the contract
        let chainIds = [chainId];
        for (const chain of data.chains) {
          chainIds.push(chain.chain_id)
        }

        // Enable Keplr using many chainIds
        await window.keplr.enable(chainIds);

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

        const offlineSigner = window.getOfflineSigner(chainId);

        const accounts = await offlineSigner.getAccounts();

        commit("setUserAddress", accounts[0].address);
        commit("setUserPubKey", accounts[0].pubkey);

        const signingClient = await SigningCosmWasmClient.connectWithSigner(
          process.env.VUE_APP_RPC,
          offlineSigner,
          // other options
          {
            registry,
            aminoTypes
          }
        );
        console.log("signer", signingClient)
        commit("setUserSigner", signingClient);
        console.log("User initialized succesfully")
      }
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

      // Contract Balance
      const contractBalance = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {
          user_balance: {
            address: state.user.address
          }
        }
      );
      commit("setUserContractBalance", contractBalance.balance);

      // Registrations
      const registrations = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {
          user_registrations: {
            address: state.user.address
          }
        }
      );
      commit("setUserRegistrations", registrations.user_chain_registrations);
      console.log("registrations fetched:", registrations)

      // Start iterating registrations to get rewards and allocated amount
      let userRewards = [];
      for (const registration of registrations.user_chain_registrations) {
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
        console.log("calculateReward", calculateReward)
        userRewards.push({
          chain_id: registration.chain_id,
          calculated_reward: calculateReward
        });
      }
      console.log("userRewards fetched:", userRewards)

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

        console.log("state.user.pubkey", state.user.pubkey)


        // take or derive the user remote address for that chain-id
        const remoteAddress = userRegistration
          ? userRegistration.remote_address // if user is registered
          : mxChainUtils.methods.deriveAddress(chain.chain_id, state.user.pubkey); // if not registered
        console.log(`remoteAddress for ${chain.chain_id}: ${remoteAddress}`)

        const chainApi = chainsApis.find(c => c.chainId === chain.chain_id);
        const delegResp = await axios.get(`${chainApi.rest}/cosmos/staking/v1beta1/delegations/${remoteAddress}`)
        if (!chain) {
          continue
        }
        delegations.push({
          chain_id: chain.chain_id,
          delegations: delegResp.data.delegation_responses
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

    async fetchDueUserChainRegistrations({state, commit}) {
      if (!state.user.querier) {
        console.error("Querier is not initialized");
        return;
      }

      // Use CosmWasmClient for the query
      const data = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {
          due_user_chain_registrations: {
            delegators_amount: 100 // TODO: Add delegators amount
          }
        }
      );

      // iterate data.due_user_chain_registrations and use .chain_id to get chain .autocompound_cost from appSupportedChains
      // foreach item we want to extend the object with that property
      data.due_user_chain_registrations.forEach((item) => {
        const chain = state.app.supportedChains.find(c => c.chain_id === item.chain_id);
        item.autocompound_cost = chain.autocompound_cost;
      });

      commit("setAppDueUserChainRegistrations", data.due_user_chain_registrations);
    },
  },

  modules: {},
});
