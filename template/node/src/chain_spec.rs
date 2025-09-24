


use std::collections::BTreeMap;
use sp_core::{Pair, Public, H160, sr25519, ed25519};
use sp_runtime::traits::Verify;
//use fp_evm::GenesisAccount;
use sp_core::ecdsa;

//use std::{collections::BTreeMap, str::FromStr};

use hex_literal::hex;
// Substrate
use sc_chain_spec::{ChainType, Properties};
//use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
//use sp_core::{Pair, Public, H160, U256, sr25519, ed25519};
//use sp_runtime::traits::{Verify};
// Agregar al inicio del archivo:
//use sp_core::hashing::keccak_256;


// Tokfin runtime types
use tokfin_runtime::{AccountId, Balance, SS58Prefix, Signature, WASM_BINARY};
//use tokfin_runtime::SessionKeys;
//use fp_account::EthereumSigner;
/*
pub fn get_eth_account_from_seed(seed: &str) -> AccountId {
    // Convierte un sr25519::Pair a una cuenta Ethereum (H160)
    EthereumSigner::from(sr25519::Pair::from_string(&format!("//{}", seed), None).unwrap().public())
        .into_account()
}
*/
//use pallet_evm::GenesisConfig as EVMConfig;
//use fp_evm::GenesisAccount;

//use sp_core::{ecdsa, H160};
//use sp_runtime::traits::IdentifyAccount;
/*
pub fn get_eth_account_id_from_seed(seed: &str) -> H160 {
    let pubkey = get_from_seed::<ecdsa::Public>(seed);
    H160::from_slice(&sp_io::hashing::keccak_256(&pubkey.0)[12..])
}
*/
//pub fn get_eth_account_from_seed(seed: &str) -> EthereumSigner {
//    let ecdsa_pair = sp_core::ecdsa::Pair::from_string(&format!("//{}", seed), None).unwrap();
//    EthereumSigner::from(ecdsa_pair.public())
//}

/// Specialized `ChainSpec` using JSON patch.
pub type ChainSpec = sc_service::GenericChainSpec;

/// Generate a crypto pair from seed.
/*
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}
*/

#[allow(dead_code)]
type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
#[allow(dead_code)]
pub fn get_account_id_from_seed(seed: &str) -> AccountId {
    let ecdsa_pair = sp_core::ecdsa::Pair::from_string(&format!("//{}", seed), None).unwrap();
    let public_bytes = ecdsa_pair.public().0;
    // Usar hash keccak256 para obtener dirección Ethereum
    use sp_core::hashing::keccak_256;
    let hash = keccak_256(&public_bytes);
    AccountId::from(H160::from_slice(&hash[12..]))
}
/*
pub fn get_account_id_from_seed(seed: &str) -> AccountId {
    AccountId::from(
        sp_core::ecdsa::Pair::from_string(&format!("//{}", seed), None)
            .unwrap()
            .public()
            .as_array_ref()
    )
}
*/
/*
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}
*/


use tokfin_runtime::opaque::SessionKeys;

fn session_keys(babe: BabeId, grandpa: GrandpaId) -> SessionKeys {
    SessionKeys { babe, grandpa }
}

/// Generate Babe/Grandpa authority keys from seed name.
/*
pub fn authority_keys_from_seed(seed: &str) -> (AccountId, BabeId, GrandpaId, SessionKeys) {
    let account_id = get_account_id_from_seed::<sr25519::Public>(seed);
    let babe = get_from_seed::<BabeId>(seed);
    let grandpa = get_from_seed::<GrandpaId>(seed);
    let session_keys = SessionKeys { babe: babe.clone(), grandpa: grandpa.clone() };

    (account_id, babe, grandpa, session_keys)
}

pub fn authority_keys_from_seed(seed: &str) -> (AccountId, BabeId, GrandpaId) {
    let account_id = get_eth_account_from_seed(seed);
    let babe_id = sr25519::Pair::from_string(&format!("//{}", seed), None).unwrap().public().unchecked_into();
    let grandpa_id = ed25519::Pair::from_string(&format!("//{}", seed), None).unwrap().public().unchecked_into();
    let session_keys = SessionKeys { babe: babe.clone(), grandpa: grandpa.clone() };

    (account_id, babe, grandpa, session_keys)
}
*/

pub fn authority_keys_from_seed(seed: &str) -> (BabeId, GrandpaId) {
    let sr25519_pair = sr25519::Pair::from_string(&format!("//{}", seed), None).unwrap();
    let ed25519_pair = ed25519::Pair::from_string(&format!("//{}", seed), None).unwrap();
    
    (
        sr25519_pair.public().into(),
        ed25519_pair.public().into(),
    )
}

fn properties() -> Properties {
    let mut properties = Properties::new();
    properties.insert("tokenSymbol".into(), "TKF".into());
    properties.insert("tokenDecimals".into(), 18.into());
    properties.insert("ss58Format".into(), SS58Prefix::get().into());
    properties
}


const UNITS: Balance = 1_000_000_000_000_000_000; // 10^18

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
            // vec![(get_account_id_from_seed("Alice"), authority_keys_from_seed("Alice").0, authority_keys_from_seed("Alice").1)],
            // vec![authority_keys_from_seed("Alice")],
            //  vec![(authority_keys_from_seed("Alice").0,1)],
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
            vec![{
                let acc = get_account_id_from_seed("Alice");
                let (babe_id, grandpa_id) = authority_keys_from_seed("Alice");
                let session_keys = session_keys(babe_id.clone(), grandpa_id.clone());
                (acc, babe_id, grandpa_id, session_keys)
            }],
            // vec![(get_account_id_from_seed("Alice"), authority_keys_from_seed("Alice").0, authority_keys_from_seed("Alice").1)],
            // vec![authority_keys_from_seed("Alice"),authority_keys_from_seed("Bob"),],
            42,     // EVM chain ID
            false,  // manual seal disabled
        ))
        .build()
}

/// Build the JSON patch for genesis.
/// NOTE: keys use the same camelCase as in other pallets (evmChainId, manualSeal, etc.)
fn testnet_genesis_json(
    sudo_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    _initial_authorities: Vec<(AccountId, BabeId, GrandpaId, SessionKeys)>,
    chain_id: u64,
    enable_manual_seal: bool,
) -> serde_json::Value {
    // EVM precompiles/accounts example
    let evm_accounts = {
        //let mut map = BTreeMap::new();
        let mut map = BTreeMap::<H160, fp_evm::GenesisAccount>::new();
        map
    };

    // Initial balances
    let balances: Vec<(AccountId, Balance)> = endowed_accounts
        .iter()
        .cloned()
//        .map(|acc| (acc, 1_000 * UNITS))
        .map(|acc| (acc, 1_000_000_000_000_000_000u128))
        .collect();

    // Tokfin assets (pallet-assets instanced as TokfinAssets in the runtime)
    // - 1: TKFr (Reputation)
    // - 2: TKFe (Equity)
    let assets = serde_json::json!({
        "assets": [
            [1, sudo_key, true, 1],
         [2, sudo_key, true, 1]
        ],
        "metadata": [
            [1, b"Reputation Token".to_vec(), b"TKFr".to_vec(), 0],
            [2, b"Equity Token".to_vec(), b"TKFe".to_vec(), 0]
        ],
        "accounts": [
            [1, sudo_key, 1_000_000_000_000_000_000u128], // 1e18
            [2, sudo_key, 80_000_000u128]
        ],
        "nextAssetId": 3
    });

    serde_json::json!({
        "sudo": { "key": sudo_key },
        "balances": { "balances": balances },
        //"babe": { "authorities": initial_authorities.iter().map(|x| x.0.clone()).collect::<Vec<_>>() },
        //"babe": { "authorities": initial_authorities.iter().map(|x| (x.0.clone(), 1)).collect::<Vec<_>>() },
        //"grandpa": { "authorities": initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect::<Vec<_>>() },
       "babe": { "epochConfig": Some(tokfin_runtime::BABE_GENESIS_EPOCH_CONFIG),},
        "grandpa": {},

//*********************
//        "session": {
//            "keys": initial_authorities.iter().map(|x| {
//                let acc: AccountId = x.0.clone();
//                (acc.clone(), acc, session_keys(x.0.clone(), x.1.clone()))
//            }).collect::<Vec<_>>()
//        },
//*********************
        "session" :{ 
            "keys": vec![{
                    let acc = get_account_id_from_seed("Alice");
                    let (babe_id, grandpa_id) = authority_keys_from_seed("Alice");
                    (
                        acc.clone(),
                        acc,
                        session_keys(babe_id, grandpa_id),
                    )
                }],

/*
            "keys": initial_authorities
                    .iter()
                    .map(|x| {
                        (
                            x.0.clone(), // account  
                            x.0.clone(), // validator
                            tokfin_runtime::opaque::SessionKeys {
                                babe: x.0.clone(),
                                grandpa: x.1.clone(),
                            }
                        )
                    })
                    .collect::<Vec<_>>(),

            "keys": initial_authorities.iter()
                .enumerate()
                .map(|(i, x)| {
                    let acc: AccountId = endowed_accounts[i].clone();
                    (
                        acc.clone(),  // stash
                        acc,          // controller
                        session_keys(x.0.clone(), x.1.clone()),
                    )
                })
                .collect::<Vec<(AccountId, AccountId, SessionKeys)>>(),
*/
        },
//*********************************

        // Frontier/evm
        "evmChainId": { "chainId": chain_id },
        "evm": { "accounts": evm_accounts },
        "ethereum": {},          // default
        "baseFee": {},           // default
        "transactionPayment": {},// default

        // Manual seal (if runtime exposes it)
        "manualSeal": { "enable": enable_manual_seal },

        // Tokfin assets
        "tokfinAssets": assets
    })
}
