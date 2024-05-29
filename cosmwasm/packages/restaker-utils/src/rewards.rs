use std::{collections::HashMap, str::FromStr};
use cosmwasm_std::{Coin, Decimal256, Deps, Env, StdError, StdResult, Uint128, Uint256};
use crate::types::{DelegatorStartingInfo, ValidatorHistoricalRewards};

pub fn calculate_delegation_rewards(
    env: Env,
    deps: Deps,
    starting_info: DelegatorStartingInfo,
    // slash_events: &[(u64, Decimal)],
    current_shares: Decimal256,   // the user shares
    all_shares: Decimal256, // all the delegators total amount of shares staked to the validator
    validator_tokens: Uint128,
    starting_val_hist_rewards: ValidatorHistoricalRewards,
    ending_val_hist_rewards: ValidatorHistoricalRewards,
) -> StdResult<Vec<Coin>> {
    deps.api.debug(format!(
        "calculate_delegation_rewards: starting_info: {:?}",
        starting_info,
    ).as_str());
    deps.api.debug(format!(
        "calculate_delegation_rewards: current_shares: {:?}",
        current_shares,
    ).as_str());
    deps.api.debug(format!(
        "calculate_delegation_rewards: validator_shares: {:?}",
        all_shares,
    ).as_str());
    deps.api.debug(format!(
        "calculate_delegation_rewards: validator_tokens: {:?}",
        validator_tokens,
    ).as_str());
    deps.api.debug(format!(
        "calculate_delegation_rewards: starting_val_hist_rewards: {:?}",
        starting_val_hist_rewards,
    ).as_str());
    deps.api.debug(format!(
        "calculate_delegation_rewards: ending_val_hist_rewards: {:?}",
        ending_val_hist_rewards,
    ).as_str());
    
    // init rewards to zero
    let mut rewards: Vec<Coin> = vec![];
    let ending_height = env.block.height;

    // TODO: This check as in the go x/distribution module
    if starting_info.height == ending_height {
        // started this height, no rewards yet
        // so we early return an empty vector
        return Ok(vec![]);
    }

    // fetch starting info for delegation
    let starting_period = starting_info.previous_period; // it should be mut if slashing calculation is implemented

    // // Iterate through slashes and withdraw with calculated staking for
    // // distribution periods. These period offsets are dependent on *when* slashes
    // // happen - namely, in BeginBlock, after rewards are allocated...
    // // Slashes which happened in the first block would have been before this
    // // delegation existed, UNLESS they were slashes of a redelegation to this
    // // validator which was itself slashed (from a fault committed by the
    // // redelegation source validator) earlier in the same BeginBlock.
    // // Slashes this block happened after reward allocation, but we have to account
    // // for them for the stake sanity check below.
    // for &(height, fraction) in slash_events.iter() {
    //     if height > starting_height && height <= ending_height {
    //         rewards += calculate_delegation_rewards_between(starting_period, height, stake)?;
    //         stake = stake * (Decimal256::one().checked_sub(fraction)?);
    //         starting_period = height;
    //     }
    //     // TODO: Just Go code for reference
    //     // if endingHeight > startingHeight {
    //     //     k.IterateValidatorSlashEventsBetween(ctx, del.GetValidatorAddr(), startingHeight, endingHeight,
    //     //         func(height uint64, event types.ValidatorSlashEvent) (stop bool) {
    //     //             endingPeriod := event.ValidatorPeriod
    //     //             if endingPeriod > startingPeriod {
    //     //                 rewards = rewards.Add(k.calculateDelegationRewardsBetween(ctx, val, startingPeriod, endingPeriod, stake)...)
    //     //                 // Note: It is necessary to truncate so we don't allow withdrawing
    //     //                 // more rewards than owed.
    //     //                 stake = stake.MulTruncate(math.LegacyOneDec().Sub(event.Fraction))
    //     //                 startingPeriod = endingPeriod
    //     //             }
    //     //             return false
    //     //         },
    //     //     )
    //     // }
    // }

    // A total stake sanity check; Recalculated final stake should be less than or
    // equal to current stake here. We cannot use Equals because stake is truncated
    // when multiplied by slash fractions (see above). We could only use equals if
    // we had arbitrary-precision rationals.
    let current_stake = tokens_from_shares(current_shares, validator_tokens, all_shares);

    let stake = Uint256::from_str(&starting_info.stake)?;
    let mut stake_decimal = Decimal256::from_atomics(stake, 18).unwrap();

    // Final stake sanity check
    if stake_decimal > current_stake {
        // AccountI for rounding inconsistencies between:
        //
        //     currentStake: calculated as in staking with a single computation
        //     stake:        calculated as an accumulation of stake
        //                   calculations across validator's distribution periods
        //
        // These inconsistencies are due to differing order of operations which
        // will inevitably have different accumulated rounding and may lead to
        // the smallest decimal place being one greater in stake than
        // currentStake. When we calculated slashing by period, even if we
        // round down for each slash fraction, it's possible due to how much is
        // being rounded that we slash less when slashing by period instead of
        // for when we slash without periods. In other words, the single slash,
        // and the slashing by period could both be rounding down but the
        // slashing by period is simply rounding down less, thus making stake >
        // currentStake
        //
        // A small amount of this error is tolerated and corrected for,
        // however any greater amount should be considered a breach in expected
        // behaviour.

        // Assuming a small margin of error, this was marginOfErr := sdk.SmallestDec().MulInt64(3)
        let margin_of_err = Decimal256::raw(3u128);

        if stake_decimal <= current_stake + margin_of_err {
            stake_decimal = current_stake;
        } else {
            return Err(StdError::generic_err(format!(
                "Calculated final stake greater than current stake. Final stake: {}, Current stake: {}",
                stake_decimal, current_stake
            )));
        }
    }

    // Calculate rewards for the final period
    rewards = add_rewards(
        rewards,
        calculate_delegation_rewards_between(
            starting_period,
            ending_height,
            starting_val_hist_rewards,
            ending_val_hist_rewards,
            stake_decimal,
        )?,
    );
    
    // Clean up amounts to the correct precision
    for coin in &mut rewards {
        let amount_as_dec = Decimal256::from_atomics(coin.amount.u128(), 18).unwrap();
        coin.amount = Uint128::try_from(amount_as_dec.to_uint_floor())?;
    }

    Ok(rewards)
}

fn add_rewards(mut rewards: Vec<Coin>, rewards_to_sum: Vec<Coin>) -> Vec<Coin> {
    // Use a HashMap to track existing rewards by denom
    let mut rewards_map: HashMap<String, usize> = HashMap::new();

    // Populate the map with existing rewards
    for (index, coin) in rewards.iter().enumerate() {
        rewards_map.insert(coin.denom.clone(), index);
    }

    // Merge or add new rewards
    for coin in rewards_to_sum {
        if let Some(&index) = rewards_map.get(&coin.denom) {
            rewards[index].amount += coin.amount;
        } else {
            rewards_map.insert(coin.denom.clone(), rewards.len());
            rewards.push(coin);
        }
    }

    rewards
}

fn sub_rewards(mut rewards: Vec<Coin>, rewards_to_subtract: Vec<Coin>) -> StdResult<Vec<Coin>> {
    // Use a HashMap to track existing rewards by denom
    let mut rewards_map: HashMap<String, usize> = HashMap::new();

    // Populate the map with existing rewards
    for (index, coin) in rewards.iter().enumerate() {
        rewards_map.insert(coin.denom.clone(), index);
    }

    // Subtract rewards
    for coin in rewards_to_subtract {
        let index = rewards_map
            .get(&coin.denom)
            .ok_or(StdError::generic_err(format!(
                "No rewards found for denom: {}",
                coin.denom
            )))?;

        // Use checked_sub to ensure no negative values
        rewards[*index].amount = rewards[*index].amount.checked_sub(coin.amount)?;
    }

    // Remove any coins that have zero amount
    rewards.retain(|coin| coin.amount > Uint128::zero());

    Ok(rewards)
}

// Mock implementation of calculate_delegation_rewards_between
fn calculate_delegation_rewards_between(
    starting_period: u64,
    ending_period: u64,
    starting_val_hist_rewards: ValidatorHistoricalRewards,
    ending_val_hist_rewards: ValidatorHistoricalRewards,
    stake: Decimal256,
) -> StdResult<Vec<Coin>> {
    // sanity check
    if starting_period > ending_period {
        panic!("starting_period cannot be greater than ending_period");
    }

    // sanity check
    if stake.is_zero() {
        panic!("stake should not be zero");
    }

    // Sub starting to ending val historic rewards, we check that we have no negative via checked_sub() inside the sub_rewards
    let subtracted_rewards = sub_rewards(
        ending_val_hist_rewards.cumulative_reward_ratio,
        starting_val_hist_rewards.cumulative_reward_ratio,
    )?;

    Ok(subtracted_rewards)
}

fn tokens_from_shares(
    shares: Decimal256,           // the user shares. originally an sdk.Dec type in Go
    validator_tokens: Uint128, // tokens define the delegated tokens (incl. self-delegation).
    all_shares: Decimal256, // all_shares defines total shares issued to a validator's delegators. originally an sdk.Dec type in Go.
) -> Decimal256 {
    if all_shares.is_zero() {
        Decimal256::zero()
    } else {
        shares * Decimal256::from_atomics(validator_tokens.u128(), 0).unwrap() / all_shares
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env};

    use crate::types::ValidatorHistoricalRewards;

    use super::*;

    // We assign 500 tokens + 500 shares to the user, and we mock 500 tokens/shares more to simulate another delegator.
    const MOCK_CURRENT_TOKENS: u128 = 500; // Mocked current shares of the delegator (single user which we are computing rewards for)
    const MOCK_CURRENT_SHARES: u128 = 500; // Mocked current shares of the delegator (single user which we are computing rewards for)
    const MOCK_VALIDATOR_SHARES: u128 = 1000; // Mocked current shares of the delegators (all of them combined)
    const MOCK_VALIDATOR_TOKENS: u128 = 1000; // Mocked validator tokens (this is how in the x/delegation_test.go)

    #[test]
    fn test_calculate_rewards_basic() {
        let env = mock_env();

        // Test variables from mocks
        let starting_info = DelegatorStartingInfo {
            previous_period: 1,
            stake: MOCK_CURRENT_TOKENS.to_string(),
            height: 1,
        }; // Mocked starting stake of the delegator
        let current_shares = Decimal256::raw(MOCK_CURRENT_SHARES);
        let delegator_shares =Decimal256::raw(MOCK_VALIDATOR_SHARES);
        let validator_tokens = Uint128::new(MOCK_VALIDATOR_TOKENS);
        // Mock the validator historical rewards to generate some rewards for the delegator
        let starting_val_hist_rewards = ValidatorHistoricalRewards {
            cumulative_reward_ratio: vec![Coin {
                denom: "untrn".to_string(),
                amount: Uint128::new(0),
            }],
            reference_count: 2,
        };
        let ending_val_hist_rewards = ValidatorHistoricalRewards {
            cumulative_reward_ratio: vec![Coin {
                denom: "untrn".to_string(),
                amount: Uint128::new(1000),
            }],
            reference_count: 2,
        };

        // // Mocking slash events
        // let slash_events: &[(u64, Decimal)] = &[
        //     (150, Decimal256::percent(10)), // Example slash event at height 150 with a 10% slash
        //     (180, Decimal256::percent(5)),  // Another example at height 180 with a 5% slash
        // ];
        let deps = mock_dependencies();

        let result = calculate_delegation_rewards(
            env,
            deps.as_ref(),
            starting_info,
            // slash_events,
            current_shares,
            delegator_shares,
            validator_tokens,
            starting_val_hist_rewards,
            ending_val_hist_rewards,
        )
        .unwrap();

        // We assert the user has now the same 500 tokens from before, plus 500 tokens from the rewards (50% of the total rewards that was 1000 tokens)
        assert_eq!(
            result,
            vec![Coin {
                denom: "untrn".to_string(),
                amount: Uint128::new(1000)
            }]
        ); // Adjust this value based on expected rewards calculation
    }

    // TODO: Tests for calculate_delegation_rewards_between

    // #[test]
    // fn test_calculate_delegation_rewards_ko() {
    //     let env = mock_env();
    //
    //     // Test variables from mocks
    //     let starting_info = DelegatorStartingInfo {
    //         previous_period: 1,
    //         stake: MOCK_CURRENT_TOKENS.to_string(),
    //         height: 1,
    //     }; // Mocked starting stake of the delegator
    //     let current_shares = Decimal256::new(Uint128::new(MOCK_CURRENT_SHARES));
    //     let delegator_shares = Decimal256::new(Uint128::new(MOCK_VALIDATOR_SHARES));
    //     let validator_tokens = Uint128::new(MOCK_VALIDATOR_TOKENS);
    //     let starting_val_hist_rewards = ValidatorHistoricalRewards {
    //         cumulative_reward_ratio: vec![Coin {
    //             denom: "untrn".to_string(),
    //             amount: Uint128::new(1000),
    //         }],
    //         reference_count: 2,
    //     };
    //     let ending_val_hist_rewards = ValidatorHistoricalRewards {
    //         cumulative_reward_ratio: vec![Coin {
    //             denom: "untrn".to_string(),
    //             amount: Uint128::new(2000),
    //         }],
    //         reference_count: 2,
    //     };
    //
    //     // // Mocking slash events
    //     // let slash_events: &[(u64, Decimal)] = &[
    //     //     (150, Decimal256::percent(10)), // Example slash event at height 150 with a 10% slash
    //     //     (180, Decimal256::percent(5)),  // Another example at height 180 with a 5% slash
    //     // ];
    //
    //     calculate_delegation_rewards(
    //         env,
    //         starting_info,
    //         // slash_events,
    //         current_shares,
    //         delegator_shares,
    //         validator_tokens,
    //         starting_val_hist_rewards,
    //         ending_val_hist_rewards,
    //     )
    //     .unwrap_err();
    // }

    #[test]
    fn test_add_rewards() {
        // Test case 1: Merging with no overlap in denominations
        let rewards = vec![
            Coin {
                denom: "denom1".to_string(),
                amount: Uint128::from(100u128),
            },
            Coin {
                denom: "denom2".to_string(),
                amount: Uint128::from(200u128),
            },
        ];

        let asd = vec![Coin {
            denom: "denom3".to_string(),
            amount: Uint128::from(300u128),
        }];

        let expected = vec![
            Coin {
                denom: "denom1".to_string(),
                amount: Uint128::from(100u128),
            },
            Coin {
                denom: "denom2".to_string(),
                amount: Uint128::from(200u128),
            },
            Coin {
                denom: "denom3".to_string(),
                amount: Uint128::from(300u128),
            },
        ];

        let result = add_rewards(rewards, asd);
        assert_eq!(result, expected);

        // Test case 2: Merging with overlap in denominations
        let rewards = vec![
            Coin {
                denom: "denom1".to_string(),
                amount: Uint128::from(100u128),
            },
            Coin {
                denom: "denom2".to_string(),
                amount: Uint128::from(200u128),
            },
        ];

        let asd = vec![
            Coin {
                denom: "denom1".to_string(),
                amount: Uint128::from(50u128),
            },
            Coin {
                denom: "denom3".to_string(),
                amount: Uint128::from(300u128),
            },
        ];

        let expected = vec![
            Coin {
                denom: "denom1".to_string(),
                amount: Uint128::from(150u128),
            },
            Coin {
                denom: "denom2".to_string(),
                amount: Uint128::from(200u128),
            },
            Coin {
                denom: "denom3".to_string(),
                amount: Uint128::from(300u128),
            },
        ];

        let result = add_rewards(rewards, asd);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sub_rewards() {
        // Test case 1: Normal subtraction
        let rewards = vec![
            Coin {
                denom: "denom1".to_string(),
                amount: Uint128::from(100u128),
            },
            Coin {
                denom: "denom2".to_string(),
                amount: Uint128::from(200u128),
            },
        ];

        let rewards_to_subtract = vec![Coin {
            denom: "denom1".to_string(),
            amount: Uint128::from(50u128),
        }];

        let result = sub_rewards(rewards.clone(), rewards_to_subtract);
        assert!(result.is_ok());
        let updated_rewards = result.unwrap();
        assert_eq!(
            updated_rewards,
            vec![
                Coin {
                    denom: "denom1".to_string(),
                    amount: Uint128::from(50u128),
                },
                Coin {
                    denom: "denom2".to_string(),
                    amount: Uint128::from(200u128),
                },
            ]
        );

        // Test case 2: Subtracting more than available
        let rewards_to_subtract = vec![Coin {
            denom: "denom1".to_string(),
            amount: Uint128::from(150u128),
        }];

        let result = sub_rewards(rewards.clone(), rewards_to_subtract);
        assert!(result.is_err());

        // Test case 3: Denom not found
        let rewards_to_subtract = vec![Coin {
            denom: "denom3".to_string(),
            amount: Uint128::from(50u128),
        }];

        let result = sub_rewards(rewards.clone(), rewards_to_subtract);
        assert!(result.is_err());
        if let Err(StdError::GenericErr { msg, .. }) = result {
            assert_eq!(msg, "No rewards found for denom: denom3");
        }

        // Test case 4: Exact subtraction to zero
        let rewards = vec![
            Coin {
                denom: "denom1".to_string(),
                amount: Uint128::from(100u128),
            },
            Coin {
                denom: "denom2".to_string(),
                amount: Uint128::from(200u128),
            },
        ];

        let rewards_to_subtract = vec![Coin {
            denom: "denom1".to_string(),
            amount: Uint128::from(100u128),
        }];

        let result = sub_rewards(rewards.clone(), rewards_to_subtract);
        assert!(result.is_ok());
        let updated_rewards = result.unwrap();
        assert_eq!(
            updated_rewards,
            vec![Coin {
                denom: "denom2".to_string(),
                amount: Uint128::from(200u128),
            }]
        );
    }

    #[test]
    fn test_tokens_from_shares() {
        // Test case 1: Normal case
        // 10 shares out of 50 total shares, with 100 validator tokens
        // should result in 10 * 100 / 50 = 20.
        let shares = Decimal256::from_str("10.0").unwrap();
        let validator_tokens = Uint128::from(100u128);
        let delegator_shares = Decimal256::from_str("50.0").unwrap(); // this includes also the 10.0 shares from the user
        let expected = Decimal256::from_str("20.0").unwrap();
        let result = tokens_from_shares(shares, validator_tokens, delegator_shares);
        assert_eq!(result, expected);

        // Test case 2: Zero shares
        // Zero shares should always result in zero tokens
        let shares = Decimal256::from_str("0.0").unwrap();
        let validator_tokens = Uint128::from(100u128);
        let delegator_shares = Decimal256::from_str("50.0").unwrap();
        let expected = Decimal256::from_str("0.0").unwrap();
        let result = tokens_from_shares(shares, validator_tokens, delegator_shares);
        assert_eq!(result, expected);

        // Test case 3: Zero validator tokens
        // 10 shares out of 50 total shares, with 0 validator tokens
        // should result in 0 tokens even if it is kinda impossible to happen.
        let shares = Decimal256::from_str("10.0").unwrap();
        let validator_tokens = Uint128::from(0u128);
        let delegator_shares = Decimal256::from_str("50.0").unwrap();
        let expected = Decimal256::from_str("0.0").unwrap();
        let result = tokens_from_shares(shares, validator_tokens, delegator_shares);
        assert_eq!(result, expected);

        // Test case 4: Zero delegator shares
        // 10 shares out of 0 total shares, with 100 validator tokens
        // should handle the division by zero safely even if it is kinda impossible to happen.
        let shares = Decimal256::from_str("10.0").unwrap();
        let validator_tokens = Uint128::from(100u128);
        let delegator_shares = Decimal256::from_str("0.0").unwrap();
        let result = tokens_from_shares(shares, validator_tokens, delegator_shares);
        let expected = Decimal256::from_str("0.0").unwrap();
        assert_eq!(result, expected);
    }
}
