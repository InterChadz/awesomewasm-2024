use cosmos_sdk_proto::cosmos::distribution::v1beta1::{DelegatorStartingInfo, ValidatorCurrentRewards as CosmosValidatorCurrentRewards, ValidatorHistoricalRewards as CosmosValidatorHistoricalRewards};
use cosmos_sdk_proto::cosmos::staking::v1beta1::{Delegation as CosmosDelegation, Validator as CosmosValidator};
use cosmos_sdk_proto::prost::Message;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, coin, StdError};
use neutron_sdk::bindings::types::StorageValue;
use neutron_sdk::interchain_queries::types::KVReconstruct;
use neutron_sdk::NeutronError::Std;
use neutron_sdk::NeutronResult;

use crate::icq::keys::{extract_addresses_from_starting_info_key, extract_validator_address_from_validator_current_rewards_key, extract_validator_address_from_validator_historic_rewards_key};

#[cw_serde]
pub struct DelegatorStartingInfoWithValidator {
    pub previous_period: u64,
    pub stake: String,
    pub height: u64,
    
    pub delegator: String,
    pub validator: String,
}

#[cw_serde]
pub struct Delegation {
    pub delegator_address: String,
    pub validator_address: String,
    pub shares: String,
}

#[cw_serde]
pub struct Validator {
    pub operator_address: String,
    pub tokens: String,
    pub all_shares: String,
}

#[cw_serde]
pub struct ValidatorHistoricalRewards {
    pub validator: String,
    pub cumulative_reward_ratio: Vec<Coin>,
    pub reference_count: u32, 
}

#[cw_serde]
pub struct ValidatorCurrentRewards {
    pub validator: String,
    pub rewards: Vec<Coin>,
    pub period: u64,
}

#[cw_serde]
pub struct UserQueryData {
    pub delegations: Vec<Delegation>,
    pub validators: Vec<Validator>,
    pub delegator_starting_infos: Vec<DelegatorStartingInfoWithValidator>,
    pub validator_historical_rewards: Vec<ValidatorHistoricalRewards>,
    pub validator_current_rewards: Vec<ValidatorCurrentRewards>,
}

impl KVReconstruct for UserQueryData {
    fn reconstruct(storage_values: &[StorageValue]) -> NeutronResult<UserQueryData> {
        let mut user_query_data = UserQueryData {
            delegations: vec![],
            validators: vec![],
            delegator_starting_infos: vec![],
            validator_historical_rewards: vec![],
            validator_current_rewards: vec![],
        };

        for sv in storage_values.iter() {
            match sv.storage_prefix.as_str() {
                "distribution" => {
                    match sv.key[0] {
                        0x04 => {
                            let delegator_starting_info = DelegatorStartingInfo::decode(sv.value.as_slice()).unwrap();
                            let (delegator, validator) = extract_addresses_from_starting_info_key(&sv.key.as_slice()).unwrap();
                            user_query_data.delegator_starting_infos.push(DelegatorStartingInfoWithValidator{
                                previous_period: delegator_starting_info.previous_period,
                                stake: delegator_starting_info.stake,
                                height: delegator_starting_info.height,
                                delegator,
                                validator,
                            });
                        },
                        0x05 => {
                            let validator_historical_rewards = CosmosValidatorHistoricalRewards::decode(sv.value.as_slice()).unwrap();
                            let cumulative_reward_ratio = validator_historical_rewards.cumulative_reward_ratio;
                            let as_coins = cumulative_reward_ratio
                                .into_iter()
                                .map(|c| coin(c.amount.parse().unwrap(), c.denom))
                                .collect::<Vec<Coin>>();
                            let reference_count = validator_historical_rewards.reference_count;
                            let validator = extract_validator_address_from_validator_historic_rewards_key(&sv.key.as_slice()).unwrap();
                            user_query_data.validator_historical_rewards.push(ValidatorHistoricalRewards{
                                validator,
                                cumulative_reward_ratio: as_coins,
                                reference_count,
                            });
                        },
                        0x06 => {
                            let validator_current_rewards = CosmosValidatorCurrentRewards::decode(sv.value.as_slice()).unwrap();
                            let rewards = validator_current_rewards.rewards;
                            let as_coins = rewards
                                .into_iter()
                                .map(|c| coin(c.amount.parse().unwrap(), c.denom))
                                .collect::<Vec<Coin>>();
                            let period = validator_current_rewards.period;
                            let validator = extract_validator_address_from_validator_current_rewards_key(&sv.key.as_slice()).unwrap();
                            user_query_data.validator_current_rewards.push(ValidatorCurrentRewards{
                                validator,
                                rewards: as_coins,
                                period,
                            });
                        }
                        _ => return Err(Std(StdError::generic_err("Unknown storage key"))),
                    }
                },
                "staking" => {
                    match sv.key[0] {
                        0x31 => {
                            let delegation = CosmosDelegation::decode(sv.value.as_slice()).unwrap();
                            user_query_data.delegations.push(Delegation{
                                delegator_address: delegation.delegator_address,
                                validator_address: delegation.validator_address,
                                shares: delegation.shares,
                            });
                        },
                        0x21 => {
                            let validator = CosmosValidator::decode(sv.value.as_slice()).unwrap();
                            user_query_data.validators.push(Validator{
                                operator_address: validator.operator_address,
                                tokens: validator.tokens,
                                all_shares: validator.delegator_shares,
                            });
                        },
                        _ => return Err(Std(StdError::generic_err("Unknown storage key"))),
                    }
                },
                _ => return Err(Std(StdError::generic_err("Unknown storage prefix"))),
            }
        }


        Ok(user_query_data)
    }
}

#[cfg(test)]
mod tests {
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD;
    use cosmwasm_std::Binary;
    use neutron_sdk::bindings::types::StorageValue;

    use super::*;

    const DELEGATION_KEY: &str = "MRR9ywXijNTWjJEJoZJa0nIAKXiodBQ9/0wU06NFlSKP51z/q2N6sup4Vg==";
    const DELEGATION_VALUE: &str = "Ci1jb3Ntb3MxMGg5c3RjNXY2bnRnZXlnZjV4Zjk0NW5qcXE1aDMycjUzdXF1dncSNGNvc21vc3ZhbG9wZXIxOGhsNWM5eG41ZHplMmc1MHVhdzBsMm1yMDJldzU3emswYXVrdG4aHjMwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMA==";
    const VALIDATOR_KEY: &str = "IRQ9/0wU06NFlSKP51z/q2N6sup4Vg==";
    const VALIDATOR_VALUE: &str = "CjRjb3Ntb3N2YWxvcGVyMThobDVjOXhuNWR6ZTJnNTB1YXcwbDJtcjAyZXc1N3prMGF1a3RuEkMKHS9jb3Ntb3MuY3J5cHRvLmVkMjU1MTkuUHViS2V5EiIKIBCnd3RZbB+HPld6eUcUk0aG79E/BVjf2ZDeZ/BSodm2IAMqDDMwNzAwMDAwMDAwMDIeMzA3MDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwOgYKBHRlc3RKAFJKCjsKEjEwMDAwMDAwMDAwMDAwMDAwMBISMjAwMDAwMDAwMDAwMDAwMDAwGhExMDAwMDAwMDAwMDAwMDAwMBILCL7u27IGELCvkERaATByATB6ATA=";
    const DELEGATOR_STARTING_INFO_KEY: &str = "BBQ9/0wU06NFlSKP51z/q2N6sup4VhR9ywXijNTWjJEJoZJa0nIAKXiodA==";
    const DELEGATOR_STARTING_INFO_VALUE: &str = "CAUSHjMwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMBj+NQ==";


    #[test]
    fn test_reconstruct_user_query_data() {
        let delegation_key = STANDARD.decode(DELEGATION_KEY).unwrap();
        let delegation_value = STANDARD.decode(DELEGATION_VALUE).unwrap();
        let validator_key = STANDARD.decode(VALIDATOR_KEY).unwrap();
        let validator_value = STANDARD.decode(VALIDATOR_VALUE).unwrap();
        let delegator_starting_info_key = STANDARD.decode(DELEGATOR_STARTING_INFO_KEY).unwrap();
        let delegator_starting_info_value = STANDARD.decode(DELEGATOR_STARTING_INFO_VALUE).unwrap();

        let storage_values = vec![
            StorageValue {
                storage_prefix: "staking".to_string(),
                key: Binary::from(delegation_key),
                value: Binary::from(delegation_value),
            },
            StorageValue {
                storage_prefix: "staking".to_string(),
                key: Binary::from(validator_key),
                value: Binary::from(validator_value),
            },
            StorageValue {
                storage_prefix: "distribution".to_string(),
                key: Binary::from(delegator_starting_info_key),
                value: Binary::from(delegator_starting_info_value),
            },
        ];

        let user_query_data = UserQueryData::reconstruct(&storage_values).unwrap();
        assert_eq!(user_query_data.delegations.len(), 1);
        assert_eq!(user_query_data.validators.len(), 1);
        assert_eq!(user_query_data.delegator_starting_infos.len(), 1);
        assert_eq!(user_query_data.validator_historical_rewards.len(), 0);
    }
}