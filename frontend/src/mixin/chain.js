import {mapGetters} from "vuex";
import {fromBech32, toBech32, toUtf8} from "@cosmjs/encoding";
import {ripemd160, sha256} from "@cosmjs/crypto";
// import {StakeAuthorization}  from "osmojs/cosmos/staking/v1beta1/authz";
// import {MsgRevoke, MsgGrant} from "osmojs/cosmos/authz/v1beta1/tx";
import {StakeAuthorization} from "osmojs/cosmos/staking/v1beta1/authz";

const mxChain = {
  computed: {
    ...mapGetters(['userSigner', 'userSigners', 'userAddress', 'appConfig', 'userPubKey']),
  },

  methods: {
    async registerUser(chainId, userAddress, validators) {
      const derived = this.deriveAddress2(chainId, this.userAddress)
      console.log("derived on register user: ", derived)
      /** @type {import("@cosmjs/proto-signing").EncodeObject} */
      const msg = {
        typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
        value: {
          sender: this.userAddress,
          contract: process.env.VUE_APP_CONTRACT,
          msg: toUtf8(JSON.stringify({
            register_user: {
              registrations: [{
                chain_id: chainId,
                address: derived, // remote chain
                validators: validators,
              }]
            }
          })),
          funds: [
            {
              denom: process.env.VUE_APP_FEE_DENOM,
              amount: "100000"
            }
          ],
        }
      }
      return this._submitTx(msg)
    },

    async topupUserBalance(funds) {
      /** @type {import("@cosmjs/proto-signing").EncodeObject} */
      const msg = {
        typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
        value: {
          sender: this.userAddress,
          contract: process.env.VUE_APP_CONTRACT,
          msg: toUtf8(JSON.stringify({
            topup_user_balance: {}
          })),
          funds: funds,
        }
      }
      return this._submitTx(msg);
    },


    async autocompound() {
      /** @type {import("@cosmjs/proto-signing").EncodeObject} */
      const msg = {
        typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
        value: {
          sender: this.userAddress,
          contract: process.env.VUE_APP_CONTRACT,
          msg: toUtf8(JSON.stringify({
            autocompound: {
              delegators_amount: 100
            },
          })),
          funds: [],
        }
      }
      return this._submitTx(msg)
    },

    // AuthZ

    async grantAuthZ(granter, grantee, address) {
      // Construct the message payload
      console.log("grantee", grantee)
      const msg = {
        typeUrl: "/cosmos.authz.v1beta1.MsgGrant",
        value: {
          granter, // the remote user account
          grantee, // the ica account for the supported chain user is granting to
          grant: {
            authorization: {
              typeUrl: "/cosmos.staking.v1beta1.StakeAuthorization",
              value: StakeAuthorization.encode(StakeAuthorization.fromPartial({
                  allowList: {
                    address: address // this should be an array of validator addresses
                  },
                  authorizationType: 1
                }
              )).finish(),
              expiration: null
            }
          }
        }
      };
      console.log("authz grant msg", msg)

      // Submit the transaction
      return this._submitTx(msg, "testy-2");
    },

    async revokeAuthZ(granter, grantee) {
      // {
      //    "@type":"/cosmos.authz.v1beta1.MsgRevoke",
      //    "granter":"cosmos10h9stc5v6ntgeygf5xf945njqq5h32r53uquvw",
      //    "grantee":"cosmos1ewfm37s7navswuva6p2v50657uyxrurplm50edtgvuhwjf0ervkstmxa2x",
      //    "msg_type_url":"/cosmos.staking.v1beta1.MsgDelegate"
      // }
      console.log("grantee", grantee)
      const msg = {
        typeUrl: "/cosmos.authz.v1beta1.MsgRevoke",
        value: {
          granter, // the remote user account
          grantee, // the ica account for the supported chain user is granting to
          msgTypeUrl: "/cosmos.staking.v1beta1.MsgDelegate"
        }
      };
      console.log("authz revoke msg", msg)

      // Submit the transaction
      return this._submitTx(msg, "testy-2");
    },

    // Utils

    getValidatorsFromDelegations(delegations) {
      // Initialize an array to hold the validator addresses
      let validatorAddresses = [];

      // Loop through the JSON data
      delegations.forEach(chain => {
        // Check if delegations are present
        if (chain.delegations && chain.delegations.length > 0) {
          // Loop through each delegation and extract the validator address
          chain.delegations.forEach(delegation => {
            validatorAddresses.push(delegation.delegation.validator_address);
          });
        }
      });

      return validatorAddresses;
    },

    deriveAddress(chainId, pubkey) {
      const chainInfo = JSON.parse(process.env.VUE_APP_CHAINS_APIS).find(c => c.chainId === chainId)
      // Hash the public key to create the address
      const sha256Hash = sha256(pubkey);
      const ripemd160Hash = ripemd160(sha256Hash);
      console.log("derived addy", toBech32(chainInfo.prefix, ripemd160Hash))
      return toBech32(chainInfo.prefix, ripemd160Hash)
    },

    deriveAddress2(chainId, address) {
      console.log("in address", address)
      const chainInfo = JSON.parse(process.env.VUE_APP_CHAINS_APIS).find(c => c.chainId === chainId)
      const {data} = fromBech32(address);
      console.log("chainInfo.prefix", chainInfo.prefix)
      const addy = toBech32(chainInfo.prefix, data)
      console.log("out addy", addy)
      return addy
    },

    displayAmount(amount, decimals = 6) {
      return (amount / 1000000).toFixed(decimals);
    },

    async suggestChain(chainInfo) {
      await window.keplr?.experimentalSuggestChain(chainInfo);
    },

    getSuggestChainInfo(chainId, prefix, rpc, rest, symbol) {
      return {
        "chainId": chainId,
        "chainName": chainId,
        "rpc": rpc,
        "rest": rest,
        "bip44": {
          "coinType": 118
        },
        "bech32Config": {
          "bech32PrefixAccAddr": prefix,
          "bech32PrefixAccPub": prefix + "pub",
          "bech32PrefixValAddr": prefix + "valoper",
          "bech32PrefixValPub": prefix + "valoperpub",
          "bech32PrefixConsAddr": prefix + "valcons",
          "bech32PrefixConsPub": prefix + "valconspub"
        },
        "currencies": [
          {
            "coinDenom": symbol.toUpperCase(),
            "coinMinimalDenom": "u" + symbol.toLowerCase(),
            "coinDecimals": 6,
            "coinGeckoId": "cosmos"
          }
        ],
        "feeCurrencies": [
          {
            "coinDenom": symbol.toUpperCase(),
            "coinMinimalDenom": "u" + symbol.toLowerCase(),
            "coinDecimals": 6,
            "coinGeckoId": prefix,
            "gasPriceStep": {
              "low": 0.0025,
              "average": 0.025,
              "high": 0.04
            }
          }
        ],
        "stakeCurrency": {
          "coinDenom": symbol.toUpperCase(),
          "coinMinimalDenom": "u" + symbol.toLowerCase(),
          "coinDecimals": 6,
          "coinGeckoId": prefix
        }
      }
    },

    // PRIVATE

    async _submitTx(message, chainId = null) {
      let signer;
      let address;
      if (chainId != null) {
        console.log("chain != null")
        console.log(this.userSigners)
        let foundSigner = this.userSigners.find(s => s.chainId === chainId);
        console.log("signer", foundSigner)
        console.log("userSigners", this.userSigners);
        signer = foundSigner.signer;
        address = foundSigner.address;
      } else {
        signer = this.userSigner;
        address = this.userAddress;
      }

      console.log("address in submittx", address)

      console.log("SUBMIT TX MSG", message)
      const gasWanted = await signer.simulate(address, [message])
      const fee = this._calculateFee(gasWanted);
      return await signer.signAndBroadcast(address, [message], fee); // Return successful response
    },

    // This has implemented as: https://hackmd.io/@3DOBr1TJQ3mQAFDEO0BXgg/S1N09wpQp
    _calculateFee(gasWanted) {
      const gas = Math.ceil(gasWanted * 2.5);
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
