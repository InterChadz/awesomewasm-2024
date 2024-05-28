use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;

// DelegatorStartingInfo represents the starting info for a delegator reward
// period. It tracks the previous validator period, the delegation's amount of
// staking token, and the creation height (to check later on if any slashes have
// occurred). NOTE: Even though validators are slashed to whole staking tokens,
// the delegators within the validator may be left with less than a full token,
// thus sdk.Dec is used.
#[cw_serde]
pub struct DelegatorStartingInfo {
    pub previous_period: u64,
    pub stake: String, // this is always intended as native staking denom, so we use String to represent sdk.Dec coming from the SDK
    pub height: u64,
}

// ValidatorHistoricalRewards represents historical rewards for a validator.
// Height is implicit within the store key.
// Cumulative reward ratio is the sum from the zeroeth period
// until this period of rewards / tokens, per the spec.
// The reference count indicates the number of objects
// which might need to reference this historical entry at any point.
// ReferenceCount =
//    number of outstanding delegations which ended the associated period (and
//    might need to read that record)
//  + number of slashes which ended the associated period (and might need to
//  read that record)
//  + one per validator for the zeroeth period, set on initialization
#[cw_serde]
pub struct ValidatorHistoricalRewards {
    pub cumulative_reward_ratio: Vec<Coin>, // representing repeated DecCoin as Vec<Coin>
    pub reference_count: u32,
}
