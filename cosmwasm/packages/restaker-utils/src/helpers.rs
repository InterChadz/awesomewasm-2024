use cosmwasm_std::{ensure, Decimal, StdError, StdResult, Uint128};

pub fn calculate_delegation_rewards(
    starting_stake: Uint128,
    previous_period: u64,
    starting_height: u64,
    ending_height: u64,
    slash_events: &[(u64, Decimal)],
    current_shares: Uint128,
) -> StdResult<Uint128> {
    // init rewards to zero
    let mut rewards = Uint128::zero();

    // TODO: This check as in the go x/distribution module
    // if startingInfo.Height == uint64(ctx.BlockHeight()) {
    // 	// started this height, no rewards yet
    // 	return
    // }

    // fetch starting info for delegation
    let mut stake = starting_stake;
    let mut starting_period = previous_period;

    // Iterate through slashes and withdraw with calculated staking for
    // distribution periods. These period offsets are dependent on *when* slashes
    // happen - namely, in BeginBlock, after rewards are allocated...
    // Slashes which happened in the first block would have been before this
    // delegation existed, UNLESS they were slashes of a redelegation to this
    // validator which was itself slashed (from a fault committed by the
    // redelegation source validator) earlier in the same BeginBlock.
    // Slashes this block happened after reward allocation, but we have to account
    // for them for the stake sanity check below.
    for &(height, fraction) in slash_events.iter() {
        if height > starting_height && height <= ending_height {
            rewards += calculate_delegation_rewards_between(starting_period, height, stake)?;
            stake = stake * (Decimal::one().checked_sub(fraction)?);
            starting_period = height;
        }
    }

    // A total stake sanity check; Recalculated final stake should be less than or
    // equal to current stake here. We cannot use Equals because stake is truncated
    // when multiplied by slash fractions (see above). We could only use equals if
    // we had arbitrary-precision rationals.
    let current_stake = get_tokens_from_shares(current_shares);

    // Final stake sanity check
    if stake > current_stake {
        let margin_of_err = Uint128::from(3u128); // Assuming a small margin of error

        ensure!(
            stake <= current_stake + margin_of_err,
            StdError::generic_err(format!(
                "Calculated final stake greater than current stake. Final stake: {}, Current stake: {}",
                stake, current_stake
            ))
        );
        stake = current_stake;
    }

    // Calculate rewards for the final period
    rewards += calculate_delegation_rewards_between(starting_period, ending_height, stake)?;

    Ok(rewards)
}

// Mock implementation of calculate_delegation_rewards_between
fn calculate_delegation_rewards_between(
    starting_period: u64,
    ending_period: u64,
    stake: Uint128,
) -> StdResult<Uint128> {
    // sanity check
    if starting_period > ending_period {
        panic!("starting_period cannot be greater than ending_period");
    }

    // sanity check
    if stake.is_zero() {
        panic!("stake should not be zero");
    }

    // TODO: Translate logic from Go to Rust to implement the logic to calculate rewards between periods
    // This is a placeholder implementation
    Ok(Uint128::new(100))
}

fn get_tokens_from_shares(shares: Uint128) -> Uint128 {
    // Mock implementation
    shares
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mocks
    const MOCK_STARTING_HEIGHT: u64 = 100;
    const MOCK_PREVIOUS_PERIOD: u64 = 10;
    const MOCK_STARTING_STAKE: u128 = 1000; // Assuming stake is an integer value
    const MOCK_CURRENT_BLOCK_HEIGHT: u64 = 200;
    const MOCK_CURRENT_STAKE: u128 = 900; // Assuming current stake after all calculations
    const MOCK_CURRENT_SHARES: u128 = 1000; // Mocked shares of the delegator

    #[test]
    fn test_calculate_delegation_rewards_ok() {
        // Arrange: setup initial conditions and inputs
        let starting_height = MOCK_STARTING_HEIGHT;
        let previous_period = MOCK_PREVIOUS_PERIOD;
        let starting_stake = Uint128::from(MOCK_STARTING_STAKE);
        let ending_height = MOCK_CURRENT_BLOCK_HEIGHT;
        let current_shares = Uint128::from(MOCK_CURRENT_SHARES);

        // Mocking slash events
        let slash_events: &[(u64, Decimal)] = &[
            (150, Decimal::percent(10)), // Example slash event at height 150 with a 10% slash
            (180, Decimal::percent(5)),  // Another example at height 180 with a 5% slash
        ];

        // Act: call the function being tested
        let result = calculate_delegation_rewards(
            starting_stake,
            previous_period,
            starting_height,
            ending_height,
            slash_events,
            current_shares,
        )
        .unwrap();

        // Assert: check the results
        assert_eq!(result, Uint128::new(300)); // Adjust this value based on expected rewards calculation
    }

    #[test]
    fn test_calculate_delegation_rewards_ko() {
        // Arrange: setup initial conditions and inputs that cause panic
        let starting_height = MOCK_STARTING_HEIGHT;
        let previous_period = MOCK_PREVIOUS_PERIOD;
        let starting_stake = Uint128::from(1500u128); // This should cause the panic
        let ending_height = MOCK_CURRENT_BLOCK_HEIGHT;
        let current_shares = Uint128::from(MOCK_CURRENT_STAKE);

        // Mocking slash events
        let slash_events: &[(u64, Decimal)] = &[
            (150, Decimal::percent(10)), // Example slash event at height 150 with a 10% slash
            (180, Decimal::percent(5)),  // Another example at height 180 with a 5% slash
        ];

        // Act: call the function being tested
        calculate_delegation_rewards(
            starting_stake,
            previous_period,
            starting_height,
            ending_height,
            slash_events,
            current_shares,
        )
        .unwrap_err();
    }
}
