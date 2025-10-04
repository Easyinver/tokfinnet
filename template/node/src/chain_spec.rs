
use std::collections::BTreeMap;
use hex_literal::hex;
use sc_chain_spec::{ChainType, Properties};
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{Pair, H160, sr25519, ed25519};
use tokfin_runtime::{AccountId, Balance, SS58Prefix, WASM_BINARY};
use tokfin_runtime::opaque::SessionKeys;

/// Specialized `ChainSpec` using JSON patch.
pub type ChainSpec = sc_service::GenericChainSpec;

/// Generate an Ethereum-compatible account ID from seed using ECDSA
pub fn get_account_id_from_seed(seed: &str) -> AccountId {
    let ecdsa_pair = sp_core::ecdsa::Pair::from_string(&format!("//{}", seed), None)
        .expect("Failed to generate ECDSA pair from seed");
    let public_bytes = ecdsa_pair.public().0;
    use sp_core::hashing::keccak_256;
    let hash = keccak_256(&public_bytes);
    AccountId::from(H160::from_slice(&hash[12..]))
}

/// Generate BABE and GRANDPA keys from seed
pub fn authority_keys_from_seed(seed: &str) -> (AccountId, BabeId, GrandpaId) {
    // BABE usa Sr25519
    let sr25519_pair = sr25519::Pair::from_string(&format!("//{}", seed), None)
        .expect("Failed to generate Sr25519 pair");
    let babe_id: BabeId = sr25519_pair.public().into();
    
    // GRANDPA usa Ed25519
    let ed25519_pair = ed25519::Pair::from_string(&format!("//{}", seed), None)
        .expect("Failed to generate Ed25519 pair");
    let grandpa_id: GrandpaId = ed25519_pair.public().into();
    
    // AccountId derivado del mismo seed para consistencia
    let account_id = get_account_id_from_seed(seed);
    
    (account_id, babe_id, grandpa_id)
}

/// Helper to create SessionKeys
fn session_keys(babe: BabeId, grandpa: GrandpaId) -> SessionKeys {
    SessionKeys { babe, grandpa }
}

fn properties() -> Properties {
    let mut properties = Properties::new();
    properties.insert("tokenSymbol".into(), "TKF".into());
    properties.insert("tokenDecimals".into(), 18.into());
    properties.insert("ss58Format".into(), SS58Prefix::get().into());
    properties
}

const TKF: Balance = 1_000_000_000_000_000_000; // 10^18

/// Development chain spec (one authority).
pub fn development_config(enable_manual_seal: bool) -> ChainSpec {
    ChainSpec::builder(WASM_BINARY.expect("WASM not available"), Default::default())
        .with_name("Development")
        .with_id("dev")
        .with_chain_type(ChainType::Development)
        .with_properties(properties())
        .with_genesis_config_patch(testnet_genesis_json(
            // Sudo account (Alith)
            AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")),
            // Prefunded accounts
            vec![
                AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")), // Alith
                AccountId::from(hex!("3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0")), // Baltathar
                AccountId::from(hex!("798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc")), // Charleth
                AccountId::from(hex!("773539d4Ac0e786233D90A233654ccEE26a613D9")), // Dorothy
                AccountId::from(hex!("Ff64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB")), // Ethan
                AccountId::from(hex!("C0F0f4ab324C46e55D02D0033343B4Be8A55532d")), // Faith
            ],
            // Initial PoA authorities
            vec![authority_keys_from_seed("Alice")],
            // EVM chain ID
            SS58Prefix::get() as u64,
            enable_manual_seal,
        ))
        .build()
}

/// Local testnet (two authorities).
pub fn local_testnet_config() -> ChainSpec {
    ChainSpec::builder(WASM_BINARY.expect("WASM not available"), Default::default())
        .with_name("Local Testnet")
        .with_id("local_testnet")
        .with_chain_type(ChainType::Local)
        .with_properties(properties())
        .with_genesis_config_patch(testnet_genesis_json(
            // Sudo account (Alith)
            AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")),
            // Prefunded accounts
            vec![
                AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")), // Alith
                AccountId::from(hex!("3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0")), // Baltathar
                AccountId::from(hex!("798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc")), // Charleth
                AccountId::from(hex!("773539d4Ac0e786233D90A233654ccEE26a613D9")), // Dorothy
                AccountId::from(hex!("Ff64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB")), // Ethan
                AccountId::from(hex!("C0F0f4ab324C46e55D02D0033343B4Be8A55532d")), // Faith
            ],
            // Authorities
            vec![
                authority_keys_from_seed("Alice"),
                authority_keys_from_seed("Bob"),
            ],
            42,     // EVM chain ID
            false,  // manual seal disabled
        ))
        .build()
}

/// Build the JSON patch for genesis.
fn testnet_genesis_json(
    sudo_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    initial_authorities: Vec<(AccountId, BabeId, GrandpaId)>,
    chain_id: u64,
    _enable_manual_seal: bool,
) -> serde_json::Value {
    // Balances for endowed accounts
    let balances: Vec<(AccountId, Balance)> = endowed_accounts
        .iter()
        .cloned()
        .map(|acc| (acc, 1_000_000 * TKF))
        .collect();

    // Session keys: pallet_session necesita (validator_id, stash_id, session_keys)
    // Como no usamos staking, validator_id == stash_id
    let session_keys: Vec<(AccountId, AccountId, SessionKeys)> = initial_authorities
        .iter()
        .map(|(account_id, babe_id, grandpa_id)| {
            (
                account_id.clone(),
                account_id.clone(),
                session_keys(babe_id.clone(), grandpa_id.clone())
            )
        })
        .collect();

    let evm_accounts = BTreeMap::<H160, fp_evm::GenesisAccount>::new();

    serde_json::json!({
        "sudo": {
            "key": Some(sudo_key)
        },
        "balances": {
            "balances": balances
        },
        "babe": {
            "authorities": Vec::<(BabeId, u64)>::new(),
            "epochConfig": Some(tokfin_runtime::BABE_GENESIS_EPOCH_CONFIG)
        },
        "grandpa": {
            "authorities": Vec::<(GrandpaId, u64)>::new()
        },
        "session": {
            "keys": session_keys
        },
        "evmChainId": {
            "chainId": chain_id
        },
        "evm": {
            "accounts": evm_accounts
        },
        "ethereum": {},
        "tokfinAssets": {
            "assets": [
                [1u32, sudo_key, true, 1u128],
                [2u32, sudo_key, true, 1u128]
            ],
            "metadata": [
                [1u32, b"Reputation Token".to_vec(), b"TKFr".to_vec(), 0u8],
                [2u32, b"Equity Token".to_vec(), b"TKFe".to_vec(), 0u8]
            ],
            "accounts": [
                [1u32, sudo_key, 1_000_000_000_000_000_000u128],
                [2u32, sudo_key, 80_000_000u128]
            ]
        }
    })
}




/*
use std::collections::BTreeMap;
use hex_literal::hex;
use sc_chain_spec::{ChainType, Properties};
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{Pair, H160, sr25519, ed25519};
use tokfin_runtime::{AccountId, Balance, SS58Prefix, WASM_BINARY};
use tokfin_runtime::opaque::SessionKeys;

/// Specialized `ChainSpec` using JSON patch.
pub type ChainSpec = sc_service::GenericChainSpec;

/// Generate an Ethereum-compatible account ID from seed using ECDSA
pub fn get_account_id_from_seed(seed: &str) -> AccountId {
    let ecdsa_pair = sp_core::ecdsa::Pair::from_string(&format!("//{}", seed), None).unwrap();
    let public_bytes = ecdsa_pair.public().0;
    use sp_core::hashing::keccak_256;
    let hash = keccak_256(&public_bytes);
    AccountId::from(H160::from_slice(&hash[12..]))
}

/// Generate BABE and GRANDPA keys from seed
pub fn authority_keys_from_seed(seed: &str) -> (BabeId, GrandpaId) {
    let sr25519_pair = sr25519::Pair::from_string(&format!("//{}", seed), None).unwrap();
    let ed25519_pair = ed25519::Pair::from_string(&format!("//{}", seed), None).unwrap();
    (sr25519_pair.public().into(), ed25519_pair.public().into())
}

/// Helper to create SessionKeys
fn session_keys(babe: BabeId, grandpa: GrandpaId) -> SessionKeys {
    SessionKeys { babe, grandpa }
}

fn properties() -> Properties {
    let mut properties = Properties::new();
    properties.insert("tokenSymbol".into(), "TKF".into());
    properties.insert("tokenDecimals".into(), 18.into());
    properties.insert("ss58Format".into(), SS58Prefix::get().into());
    properties
}

const TKF: Balance = 1_000_000_000_000_000_000; // 10^18

/// Development chain spec (one authority).
pub fn development_config(enable_manual_seal: bool) -> ChainSpec {
    ChainSpec::builder(WASM_BINARY.expect("WASM not available"), Default::default())
        .with_name("Development")
        .with_id("dev")
        .with_chain_type(ChainType::Development)
        .with_properties(properties())
        .with_genesis_config_patch(testnet_genesis_json(
            // Sudo account (Alith)
            AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")),
            // Prefunded accounts
            vec![
                AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")), // Alith
                AccountId::from(hex!("3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0")), // Baltathar
                AccountId::from(hex!("798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc")), // Charleth
                AccountId::from(hex!("773539d4Ac0e786233D90A233654ccEE26a613D9")), // Dorothy
                AccountId::from(hex!("Ff64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB")), // Ethan
                AccountId::from(hex!("C0F0f4ab324C46e55D02D0033343B4Be8A55532d")), // Faith
            ],
            // Initial PoA authorities
            vec![{
                let acc = get_account_id_from_seed("Alice");
                let (babe_id, grandpa_id) = authority_keys_from_seed("Alice");
                let session_keys = session_keys(babe_id.clone(), grandpa_id.clone());
                (acc, babe_id, grandpa_id, session_keys)
            }],
            // EVM chain ID
            SS58Prefix::get() as u64,
            enable_manual_seal,
        ))
        .build()
}

/// Local testnet (two authorities).
pub fn local_testnet_config() -> ChainSpec {
    ChainSpec::builder(WASM_BINARY.expect("WASM not available"), Default::default())
        .with_name("Local Testnet")
        .with_id("local_testnet")
        .with_chain_type(ChainType::Local)
        .with_properties(properties())
        .with_genesis_config_patch(testnet_genesis_json(
            // Sudo account (Alith)
            AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")),
            // Prefunded accounts
            vec![
                AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")), // Alith
                AccountId::from(hex!("3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0")), // Baltathar
                AccountId::from(hex!("798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc")), // Charleth
                AccountId::from(hex!("773539d4Ac0e786233D90A233654ccEE26a613D9")), // Dorothy
                AccountId::from(hex!("Ff64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB")), // Ethan
                AccountId::from(hex!("C0F0f4ab324C46e55D02D0033343B4Be8A55532d")), // Faith
            ],
            // Authorities
            vec![
                {
                    let acc = get_account_id_from_seed("Alice");
                    let (babe_id, grandpa_id) = authority_keys_from_seed("Alice");
                    let session_keys = session_keys(babe_id.clone(), grandpa_id.clone());
                    (acc, babe_id, grandpa_id, session_keys)
                },
                {
                    let acc = get_account_id_from_seed("Bob");
                    let (babe_id, grandpa_id) = authority_keys_from_seed("Bob");
                    let session_keys = session_keys(babe_id.clone(), grandpa_id.clone());
                    (acc, babe_id, grandpa_id, session_keys)
                }
            ],
            42,     // EVM chain ID
            false,  // manual seal disabled
        ))
        .build()
}

/// Build the JSON patch for genesis.
fn testnet_genesis_json(
    sudo_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    initial_authorities: Vec<(AccountId, BabeId, GrandpaId, SessionKeys)>,
    chain_id: u64,
    _enable_manual_seal: bool,
) -> serde_json::Value {
    // Pre-calculate values outside the JSON macro
    let balances: Vec<(AccountId, Balance)> = endowed_accounts
        .iter()
        .cloned()
        .map(|acc| (acc, 1_000_000 * TKF))
        .collect();

    let session_keys: Vec<(AccountId, AccountId, SessionKeys)> = initial_authorities
        .iter()
        .map(|x| (x.0.clone(), x.0.clone(), x.3.clone()))
        .collect();

    let evm_accounts = BTreeMap::<H160, fp_evm::GenesisAccount>::new();

    serde_json::json!({
        "sudo": {
            "key": Some(sudo_key)
        },
        "balances": {
            "balances": balances
        },
        "babe": {
            "authorities": Vec::<(BabeId, u64)>::new(),
            "epochConfig": Some(tokfin_runtime::BABE_GENESIS_EPOCH_CONFIG)
        },
        "grandpa": {
            "authorities": Vec::<(GrandpaId, u64)>::new()
        },
        "session": {
            "keys": session_keys
        },
        "evmChainId": {
            "chainId": chain_id
        },
        "evm": {
            "accounts": evm_accounts
        },
        "ethereum": {},
        "tokfinAssets": {
            "assets": [
                [1u32, sudo_key, true, 1u128],
                [2u32, sudo_key, true, 1u128]
            ],
            "metadata": [
                [1u32, b"Reputation Token".to_vec(), b"TKFr".to_vec(), 0u8],
                [2u32, b"Equity Token".to_vec(), b"TKFe".to_vec(), 0u8]
            ],
            "accounts": [
                [1u32, sudo_key, 1_000_000_000_000_000_000u128],
                [2u32, sudo_key, 80_000_000u128]
            ]
        }
    })
}

*/