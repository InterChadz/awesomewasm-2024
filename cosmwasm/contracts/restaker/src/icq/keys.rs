use bech32::{Bech32, Hrp};
use byteorder::{ByteOrder, LittleEndian};
use cosmwasm_std::Binary;
use neutron_sdk::bindings::types::KVKey;
use neutron_sdk::interchain_queries::helpers::{decode_and_convert, length_prefix};
use neutron_sdk::interchain_queries::v045::types::STAKING_STORE_KEY;
use neutron_sdk::interchain_queries::v047::helpers::create_delegation_key;
use neutron_sdk::interchain_queries::v047::helpers::create_validator_key;
use neutron_sdk::NeutronResult;

pub const DISTRIBUTION_STORE_PREFIX: &str = "distribution";
const DISTRIBUTION_STORE_DELEGATOR_STARTING_INFO_PREFIX: u8 = 0x04;
const DISTRIBUTION_STORE_VALIDATOR_HISTORICAL_REWARDS_PREFIX: u8 = 0x05;
const DISTRIBUTION_STORE_VALIDATOR_CURRENT_REWARDS_PREFIX: u8 = 0x06;

pub struct ValidatorHistoricalRange {
    pub validator: String,
    pub period: u64,
}

pub fn create_all_icq_keys_for_user(
    delegator: String,
    validators: Vec<String>, 
    validator_historical_range: Option<Vec<ValidatorHistoricalRange>>,
) -> NeutronResult<Vec<KVKey>> {
    let delegation_keys = create_delegator_delegations_query_keys(
        delegator.clone(),
        validators.clone(),
    ).unwrap();
    let validator_keys = create_validator_query_keys(validators.clone()).unwrap();
    let delegator_starting_info_keys = create_delegator_starting_info_query_keys(
        delegator,
        validators.clone(),
    ).unwrap();
    let validator_current_rewards_keys = create_validator_current_rewards_query_keys(validators).unwrap();
    
    let historical_rewards_keys = if validator_historical_range.is_some() {
        create_validator_historical_rewards_query_keys(validator_historical_range.unwrap()).unwrap() 
    } else {
        vec![]
    };

    let all_keys = delegation_keys
        .into_iter()
        .chain(validator_keys.into_iter())
        .chain(delegator_starting_info_keys.into_iter())
        .chain(validator_current_rewards_keys.into_iter())
        .chain(historical_rewards_keys.into_iter())
        .collect();
    
    Ok(all_keys)
}

pub fn create_delegator_delegations_query_keys(
    delegator: String,
    validators: Vec<String>,
) -> NeutronResult<Vec<KVKey>> {
    let delegator_addr = decode_and_convert(&delegator).unwrap();

    let mut keys: Vec<KVKey> = Vec::with_capacity(validators.len());
    
    for v in validators {
        let val_addr = decode_and_convert(&v).unwrap();

        // create delegation key to get delegation structure
        keys.push(KVKey {
            path: STAKING_STORE_KEY.to_string(),
            key: Binary(create_delegation_key(&delegator_addr, &val_addr).unwrap()),
        });
    }
    
    Ok(keys)
}

pub fn create_validator_query_keys(validators: Vec<String>) -> NeutronResult<Vec<KVKey>> {
    let mut keys: Vec<KVKey> = Vec::with_capacity(validators.len());

    for v in validators {
        let val_addr = decode_and_convert(&v).unwrap();

        // create delegation key to get delegation structure
        keys.push(KVKey {
            path: STAKING_STORE_KEY.to_string(),
            key: Binary(create_validator_key(&val_addr).unwrap()),
        });
    }

    Ok(keys)
}

pub fn create_validator_historical_rewards_query_keys(validators: Vec<ValidatorHistoricalRange>) -> NeutronResult<Vec<KVKey>> {
    let mut keys: Vec<KVKey> = Vec::with_capacity(validators.len());

    for v in validators {
        let val_addr = decode_and_convert(&v.validator).unwrap();

        // create delegation key to get delegation structure
        keys.push(KVKey {
            path: DISTRIBUTION_STORE_PREFIX.to_string(),
            key: Binary(create_distribution_validator_historical_rewards_prefix_key(&val_addr, v.period).unwrap()),
        });
    }

    Ok(keys)
}

pub fn extract_validator_address_from_validator_historic_rewards_key(key: &[u8]) -> NeutronResult<String> {
    let validator_length = key[1] as usize;
    let validator_addr = &key[2..(2 + validator_length)];
    let validator_hrp = Hrp::parse("cosmosvaloper").unwrap();
    let bech32_validator_address: String = bech32::encode::<Bech32>(validator_hrp, validator_addr).unwrap();
    
    decode_and_convert(&bech32_validator_address).expect(format!("Failed to decode validator address: {}", bech32_validator_address).as_str());

    Ok(bech32_validator_address)
}

pub fn create_delegator_starting_info_query_keys(
    delegator: String,
    validators: Vec<String>,
) -> NeutronResult<Vec<KVKey>> {
    let delegator_addr = decode_and_convert(&delegator).unwrap();

    let mut keys: Vec<KVKey> = Vec::with_capacity(validators.len());

    for v in validators {
        let val_addr = decode_and_convert(&v).unwrap();

        // create delegation key to get delegation structure
        keys.push(KVKey {
            path: DISTRIBUTION_STORE_PREFIX.to_string(),
            key: Binary(create_distribution_store_delegator_starting_info_prefix_key(&delegator_addr, &val_addr).unwrap()),
        });
    }

    Ok(keys)
}

pub fn create_validator_current_rewards_query_keys(validators: Vec<String>) -> NeutronResult<Vec<KVKey>> {
    let mut keys: Vec<KVKey> = Vec::with_capacity(validators.len());

    for v in validators {
        let val_addr = decode_and_convert(&v).unwrap();

        // create delegation key to get delegation structure
        keys.push(KVKey {
            path: DISTRIBUTION_STORE_PREFIX.to_string(),
            key: Binary(create_distribution_store_validator_current_rewards_prefix_key(&val_addr).unwrap()),
        });
    }

    Ok(keys)
}

fn create_distribution_store_validator_current_rewards_prefix_key<AddrBytes: AsRef<[u8]>>(
    validator_addr: AddrBytes,
) -> NeutronResult<Vec<u8>> {
    let mut key: Vec<u8> = vec![DISTRIBUTION_STORE_VALIDATOR_CURRENT_REWARDS_PREFIX];
    key.extend_from_slice(length_prefix(validator_addr)?.as_slice());
    
    Ok(key)
}

pub fn extract_validator_address_from_validator_current_rewards_key(key: &[u8]) -> NeutronResult<String> {
    let validator_length = key[1] as usize;
    let validator_addr = &key[2..(2 + validator_length)];
    let validator_hrp = Hrp::parse("cosmosvaloper").unwrap();
    let bech32_validator_address: String = bech32::encode::<Bech32>(validator_hrp, validator_addr).unwrap();
    
    decode_and_convert(&bech32_validator_address).expect(format!("Failed to decode validator address: {}", bech32_validator_address).as_str());

    Ok(bech32_validator_address)
}

fn create_distribution_store_delegator_starting_info_prefix_key<AddrBytes: AsRef<[u8]>>(
    delegator_address: AddrBytes,
    validator_addr: AddrBytes) -> NeutronResult<Vec<u8>> {
    let mut key: Vec<u8> = vec![DISTRIBUTION_STORE_DELEGATOR_STARTING_INFO_PREFIX];
    key.extend_from_slice(length_prefix(validator_addr)?.as_slice());
    key.extend_from_slice(length_prefix(delegator_address)?.as_slice());

    Ok(key)
}

// (delegator, validator)
pub fn extract_addresses_from_starting_info_key(key: &[u8]) -> NeutronResult<(String, String)> {
    let validator_length = key[1] as usize;
    let validator_addr = &key[2..(2 + validator_length)];
    let validator_hrp = Hrp::parse("cosmosvaloper").unwrap();
    let bech32_validator_address: String = bech32::encode::<Bech32>(validator_hrp, validator_addr).unwrap();
    
    let delegator_length = key[2 + validator_length] as usize;
    let delegator_addr = &key[(2 + validator_length + 1)..(2 + validator_length + 1 + delegator_length)];
    let delegator_hrp = Hrp::parse("cosmos").unwrap();
    let bech32_delegator_address: String = bech32::encode::<Bech32>(delegator_hrp, delegator_addr).unwrap();

    decode_and_convert(&bech32_delegator_address).expect(format!("Failed to decode delegator address: {}", bech32_delegator_address).as_str());
    decode_and_convert(&bech32_validator_address).expect(format!("Failed to decode validator address: {}", bech32_validator_address).as_str());

    Ok((bech32_delegator_address, bech32_validator_address))
}

fn create_distribution_validator_historical_rewards_prefix_key<AddrBytes: AsRef<[u8]>>(
    validator_addr: AddrBytes,
    height: u64,
) -> NeutronResult<Vec<u8>> {
    let mut key: Vec<u8> = vec![DISTRIBUTION_STORE_VALIDATOR_HISTORICAL_REWARDS_PREFIX];
    key.extend_from_slice(length_prefix(validator_addr)?.as_slice());

    let mut buf = [0; 8];
    LittleEndian::write_u64(&mut buf, height);
    key.extend_from_slice(&buf);
    
    Ok(key)
}

#[cfg(test)]
mod tests {
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD;
    use cosmwasm_std::Binary;
    use neutron_sdk::interchain_queries::helpers::decode_and_convert;

    use crate::icq::keys::{create_distribution_validator_historical_rewards_prefix_key, extract_addresses_from_starting_info_key};

    const STARTING_INFO_KEY: &str = "BBQ9/0wU06NFlSKP51z/q2N6sup4VhR9ywXijNTWjJEJoZJa0nIAKXiodA==";
    const STARTING_INFO_VALIDATOR: &str = "cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn";
    const STARTING_INFO_DELEGATOR: &str = "cosmos10h9stc5v6ntgeygf5xf945njqq5h32r53uquvw";
    
    #[test]
    fn test_extract_validator_address_from_starting_info_key() {
        let binary = Binary::from_base64(&STARTING_INFO_KEY).unwrap();
        let (delegator, validator) = extract_addresses_from_starting_info_key(binary.as_slice()).unwrap();
        assert_eq!(delegator, STARTING_INFO_DELEGATOR);
        assert_eq!(validator, STARTING_INFO_VALIDATOR);

        decode_and_convert(&delegator).unwrap();
        decode_and_convert(&validator).unwrap();
    }
    
    #[test]
    fn test_create_distribution_validator_historical_rewards_prefix_key() {
        let validator = decode_and_convert(STARTING_INFO_VALIDATOR).unwrap();
        let key = create_distribution_validator_historical_rewards_prefix_key(&validator, 100).unwrap();
        assert_eq!(STANDARD.encode(&key), "BRQ9/0wU06NFlSKP51z/q2N6sup4VmQAAAAAAAAA");
    }
}
