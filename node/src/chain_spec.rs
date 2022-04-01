use node_template_runtime::{AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig, Signature, SudoConfig,
	SystemConfig, WASM_BINARY,
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use hex_literal::hex;
use sc_service::Properties;
use node_primitives::{AccountId, Balance};
use sp_core::crypto::UncheckedInto;
// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
use serde_json::json;
/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;
const DEFAULT_PROTOCOL_ID: &str = "kitty";

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		None,
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
	}
}



/// Configure initial storage state for FRAME modules.
fn kitty_testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<(AccountId, Balance)>,
	_enable_println: bool,
) -> GenesisConfig {

	let allocation_account :Vec<(AccountId, Balance)> = endowed_accounts.iter().map(|x| (x.0.clone(), x.1.clone())).collect();
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: allocation_account,
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
	}
}


pub fn kitty_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Staging wasm binary not available".to_string())?;
	let initial_authorities: Vec<(AuraId, GrandpaId)> = vec![
		(
            hex!["7248900331fe3d1dc7a5c503e870bd7a5f17b3df73bbdca440c419761a99330c"].unchecked_into(),
			hex!["67a92511dad839bd065a0183ce447273c72cc267aca1f15291d9fc02ab1a2ae1"].unchecked_into(),
		),
		(
            hex!["8e122f2afc57fae170be5027563b332273e62d50b3f9daee19ebeeb7be43b97b"].unchecked_into(),
			hex!["e9c46eb1281c9d2ed5cc0c02c9a89c8d727c386143aef5bad44fb67118ac1710"].unchecked_into(),
		),
	];

	let endownment_account:Vec<(AccountId, Balance)> = vec![
        (
			//root
            hex!["16c1e8c292b0ca968ee84d3f33de819dd3d1466ee4a5a025c4c714582e29fa26"].into(),
			10_000_000
		),
		(
            hex!["18c31f136bd4e7d942de1a291cf11bab8a9d93fc38036a7951b49c2017166c39"].into(),
			10_000_000
		),

        (
            hex!["ee83256d5fd8a4fc41b418d2ae84be2448a7458ce4ad8e88a545e47715a9e000"].into(),
			10_000_000
		),

        
	];

	let data = r#"
	{
		"ss58Format": 42,
		"tokenDecimals": 9,
		"tokenSymbol": "KIT"
	}"#;
	//let properties = serde_json::from_str(data).unwrap();

    Ok(ChainSpec::from_genesis(
        "Kitty Testnet",
        "Kitty_testnet",
        ChainType::Live,
        move || kitty_testnet_genesis(
            wasm_binary,
			initial_authorities.clone(),
            /* Sudo Account */
            hex!["16c1e8c292b0ca968ee84d3f33de819dd3d1466ee4a5a025c4c714582e29fa26"].into(),
            endownment_account.clone(),
            true,
        ),
        vec![],
		None,
        Some(DEFAULT_PROTOCOL_ID),
        None,
		Some(
			serde_json::from_str(
				"{\"tokenDecimals\": 12, \"tokenSymbol\": \"KIT\", \"SS58Prefix\": 28}",
			)
			.expect("Provided valid json map"),
		),
		None,
    ))
}
