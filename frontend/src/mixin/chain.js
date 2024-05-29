import mxToast from "./toast";
import {mapGetters} from "vuex";
import {toUtf8} from "@cosmjs/encoding";

const mxChain = {
  mixins: [mxToast],

  computed: {
    ...mapGetters(['userSigner', 'userAddress', 'appConfig']),
  },

  methods: {
    async registerUser() {
      /** @type {import("@cosmjs/proto-signing").EncodeObject} */
      const msg = {
        typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
        value: {
          sender: this.userAddress,
          contract: process.env.VUE_APP_CONTRACT,
          msg: toUtf8(JSON.stringify({
            register_user: {}
          })),
          funds: [],
        }
      }
      return this._submitTx(msg)
    },

    methods: {
  async topupUserBalance(funds) {
    /** @type {import("@cosmjs/proto-signing").EncodeObject} */
    const msg = {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: {
        sender: this.userAddress,
        contract: process.env.VUE_APP_CONTRACT,
        msg: toUtf8(JSON.stringify({
          register_user: {}
        })),
        funds: funds,
      }
    }
    return this._submitTx(msg);
  },
  // Other methods...
},

    async autocompound() {
      /** @type {import("@cosmjs/proto-signing").EncodeObject} */
      const msg = {
        typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
        value: {
          sender: this.userAddress,
          contract: process.env.VUE_APP_CONTRACT,
          msg: toUtf8(JSON.stringify({
            register_user: {}
          })),
          funds: [],
        }
      }
      return this._submitTx(msg)
    },

    // Utils

    displayAmount(amount, decimals = 6) {
      return (amount / 1000000).toFixed(decimals);
    },

    async suggestChain() {
      await window.keplr?.experimentalSuggestChain(JSON.parse(process.env.VUE_APP_CHAIN_INFO));
    },

    // PRIVATE

    async _submitTx(message) {
      const gasWanted = await this.userSigner.simulate(this.userAddress, [message])
      const fee = this._calculateFee(gasWanted);
      return await this.userSigner.signAndBroadcast(this.userAddress, [message], fee); // Return successful response
    },

    // This has implemented as: https://hackmd.io/@3DOBr1TJQ3mQAFDEO0BXgg/S1N09wpQp
    _calculateFee(gasWanted) {
      const gas = Math.ceil(gasWanted * 1.3);
      const baseFee = Number(process.env.VUE_APP_BASE_FEE)

      // baseFee * 3 doesn't seem to be necessary after v23 upgrade, but leaving that here for the moment
      const amount = Math.ceil(baseFee * gas).toString();
      return {
        amount: [{denom: process.env.VUE_APP_FEE_DENOM, amount}],
        gas: gas.toString(),
      }
    }
  }
}

export default mxChain
