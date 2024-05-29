# awesomewasm-2024

<div align="center">
  <h1 align="center">InterChadz</h1>
  <h3>ICA restaker on Neutron</h3>
  
![logo](logo.png)
</div>

💸 Given the recent news about Yieldmos being taken down, there is a need for a new restaking solution. InterChadz uses a restaking contract deployed on Neutron where users pay a fee to set up automated compounding of staking rewards.

⚙️ Built with interchian accounts + authz + Neutron Cron.

## Flow

![flow](flow.png)

### Initial registration flow

1. User sends registration tx to the contract with the network(s) they want to autocompound on and their address
2. The contract creates one ICA on each network for the user
3. Register periodic ICQ for the ICA balance (done in the callback, since we don't know the address beforehand)
4. User send authz tx (for the delegate message) for each ICA, one transaction for each network they want to autocompound on.

## Screenshots

| Landing Page                               | Restaking dashboard                        |
| ------------------------------------------ | ------------------------------------------ |
| ![Screenshot](screenshots/placeholder.png) | ![Screenshot](screenshots/placeholder.png) |

| Compounding                                | Other screenshot                           |
| ------------------------------------------ | ------------------------------------------ |
| ![Screenshot](screenshots/placeholder.png) | ![Screenshot](screenshots/placeholder.png) |

## Project setup

For detailed instructions to start the dApp, see the respective readme files:

- [Frontend installation instructions](https://github.com/InterChadz/awesomewasm-2024/blob/main/frontend/README-Vue.md)
  - The frontend can also be accessed [here](https://interchadz.vercel.app/) without installation.
- [Contracts installation instructions](https://github.com/InterChadz/awesomewasm-2024/blob/main/cosmwasm/README.md)

## Functions

### RegisterUser

This function is available on the staking dashboard as a toggle. When restaking is not enabled, users can activate it by clicking the toggle. A transaction is initiated, with `chainId`, `address` and validators as parameters. The chainid and address are available in the state, and the validators are queried from the chain.

### Autocompound

This function triggers the autocompound feature, compounding all pending rewards to the staked balances.

### TopupUserBalance

TODO

## Links

- [Vercel deployment](https://interchadz.vercel.app/)
- [Presentation slides]()
- [Demo video]()
- [Github repo](https://github.com/InterChadz/awesomewasm-2024)
- [Twitter/X profile](https://x.com/TheInterChadz)

## Team

This project was build during AwesomWasm Hackathon 2024 by:

- [Gjermund Garaba](https://x.com/GjermundGaraba)
- [magiodev](https://x.com/magiodev)
- [arjanjohan](https://x.com/arjanjohan/)
