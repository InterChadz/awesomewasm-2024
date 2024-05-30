import {mapGetters} from "vuex";
import {fromBech32, toBech32, toUtf8} from "@cosmjs/encoding";
import {ripemd160, sha256} from "@cosmjs/crypto";

const mxChain = {
  computed: {
    ...mapGetters(['userSigner', 'userAddress', 'appConfig', 'userPubKey']),
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
      // {
      //    "@type":"/cosmos.authz.v1beta1.MsgGrant",
      //    "granter":"cosmos10h9stc5v6ntgeygf5xf945njqq5h32r53uquvw",
      //    "grantee":"cosmos1jat2z0ffpn7cu50zjxk4wyy89e945pczlv9jnegt5dlwhzkdeh9quhkt0a",
      //    "grant":{
      //       "authorization":{
      //          "@type":"/cosmos.staking.v1beta1.StakeAuthorization",
      //          "max_tokens":null,
      //          "allow_list":{
      //             "address":[
      //                "cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn"
      //             ]
      //          },
      //          "authorization_type":"AUTHORIZATION_TYPE_DELEGATE"
      //       },
      //       "expiration":null
      //    }
      // }
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
              value: {
                allowList: {
                  address // this should be an array of validator addresses
                },
                authorizationType: "AUTHORIZATION_TYPE_DELEGATE"
              }
            },
            expiration: null
          }
        }
      };
      console.log("authz grant msg", msg)

      // Submit the transaction
      return this._submitTx(msg);
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
      const { data } = fromBech32(address);
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
        "rpc": "http://localhost:26657",
        "rest": "http://localhost:1317",
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
            "coinGeckoId": "neutron"
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

    async _submitTx(message) {
      console.log("SUBMIT TX MSG", message)
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
