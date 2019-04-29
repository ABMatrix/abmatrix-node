use primitives::{Ed25519AuthorityId as AuthorityId, ed25519};
use node_runtime::{
    GenesisConfig, ConsensusConfig, SessionConfig, StakingConfig, TimestampConfig,
    IndicesConfig, BalancesConfig, FeesConfig, GrandpaConfig, SudoConfig,BankConfig,
    AccountId, Perbill
};
use substrate_service::{self, Properties};
use serde_json::json;

use substrate_keystore::pad_seed;
use substrate_telemetry::TelemetryEndpoints;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialised `ChainSpec`. This is a specialisation of the general Substrate ChainSpec type.
pub type ChainSpec = substrate_service::ChainSpec<GenesisConfig>;

/// The chain specification option. This is expected to come in from the CLI and
/// is little more than one of a number of alternatives which can easily be converted
/// from a string (`--chain=...`) into a `ChainSpec`.
#[derive(Clone, Debug)]
pub enum ChainOpt {
    /// Whatever the current runtime is, with just Alice as an auth.
    Development,
    /// Whatever the current runtime is, with simple Alice/Bob auths.
    LocalTestnet,
    /// Abmatrix public testnet.
    AbmatrixTestnet,
}

impl ChainOpt {
    /// Get an actual chain config from one of the alternatives.
    pub(crate) fn load(self) -> Result<ChainSpec, String> {
        Ok(match self {
            ChainOpt::Development => development_config(),
            ChainOpt::LocalTestnet => local_testnet_config(),
            ChainOpt::AbmatrixTestnet => abmatrix_testnet_config(),
        })
    }

    pub(crate) fn from(s: &str) -> Option<Self> {
        match s {
            "dev" => Some(ChainOpt::Development),
            "local" => Some(ChainOpt::LocalTestnet),
            "" | "abmatrixtest" => Some(ChainOpt::AbmatrixTestnet),
            _ => None,
        }
    }
}

/// Helper function to generate AuthorityID from seed
pub fn get_account_id_from_seed(seed: &str) -> AccountId {
    let padded_seed = pad_seed(seed);
    // NOTE from ed25519 impl:
    // prefer pkcs#8 unless security doesn't matter -- this is used primarily for tests.
    ed25519::Pair::from_seed(&padded_seed).public().0.into()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(seed: &str) -> (AccountId, AccountId, AuthorityId) {
    let padded_seed = pad_seed(seed);
    // NOTE from ed25519 impl:
    (
        get_account_id_from_seed(&format!("{}//stash", seed)),
        get_account_id_from_seed(seed),
        ed25519::Pair::from_seed(&padded_seed).public().0.into()
    )
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
    initial_authorities: Vec<(AccountId, AccountId, AuthorityId)>,
    root_key: AccountId,
    endowed_accounts: Option<Vec<AccountId>>,
) -> GenesisConfig {
    let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
        vec![
            get_account_id_from_seed("Alice"),
            get_account_id_from_seed("Bob"),
            get_account_id_from_seed("Charlie"),
            get_account_id_from_seed("Dave"),
            get_account_id_from_seed("Eve"),
            get_account_id_from_seed("Ferdie"),
        ]
    });

    const COASE: u128 = 1_000;
    const GLUSHKOV: u128 = 1_000 * COASE;    // assume this is worth about a cent.
    const XRT: u128 = 1_000 * GLUSHKOV;

    const SECS_PER_BLOCK: u64 = 4;
    const MINUTES: u64 = 60 / SECS_PER_BLOCK;

    const ENDOWMENT: u128 = 10_000_000 * XRT;
    const STASH: u128 = 100 * XRT;

    GenesisConfig {
        consensus: Some(ConsensusConfig {
            code: include_bytes!("../runtime/wasm/target/wasm32-unknown-unknown/release/node_runtime.compact.wasm").to_vec(),
            authorities: initial_authorities.iter().map(|x| x.2.clone()).collect(),
        }),
        system: None,
        indices: Some(IndicesConfig {
            ids: endowed_accounts.clone(),
        }),
        balances: Some(BalancesConfig {
            existential_deposit: 1 * COASE,
            transfer_fee: 0,
            creation_fee: 0,
            balances: endowed_accounts.iter()
                .map(|&k| (k, ENDOWMENT))
                .chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
                .collect(),
            vesting: vec![],
        }),
        session: Some(SessionConfig {
            validators: initial_authorities.iter().map(|x| x.1.into()).collect(),
            session_length: 15,
            keys: initial_authorities.iter().map(|x| (x.1.clone(), x.2.clone())).collect::<Vec<_>>(),
        }),
        staking: Some(StakingConfig {
            current_era: 0,
            minimum_validator_count: 1,
            validator_count: 4,
            sessions_per_era: 60,
            bonding_duration: 60 * MINUTES,
            session_reward: Perbill::from_millionths(200_000),
            offline_slash: Perbill::from_millionths(1_000_000),
            current_offline_slash: 0,
            current_session_reward: 0,
            offline_slash_grace: 4,
            stakers: initial_authorities.iter().map(|x| (x.0.into(), x.1.into(), STASH)).collect(),
            invulnerables: initial_authorities.iter().map(|x| x.1.into()).collect(),
        }),
        timestamp: Some(TimestampConfig {
            period: SECS_PER_BLOCK / 2,
        }),
        sudo: Some(SudoConfig {
            key: root_key,
        }),
        grandpa: Some(GrandpaConfig {
            authorities: initial_authorities.iter().map(|x| (x.2.clone(), 1)).collect(),
        }),
        fees: Some(FeesConfig {
            transaction_base_fee: 1 * GLUSHKOV,
            transaction_byte_fee: 50 * COASE,
        }),
        bank: Some(BankConfig{
            enable_record: true,
            session_length: 10,
            reward_session_value: vec![1000,5000,10000,50000,500000],
            reward_session_factor: vec![1,2,3,4,5],
            reward_balance_value: vec![1000,5000,10000,50000,500000],
            reward_balance_factor: vec![1,2,3,4,5],
            total_despositing_balance: 0,
        })
    }
}

fn development_config_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![
            get_authority_keys_from_seed("Alice"),
        ],
        get_account_id_from_seed("Alice").into(),
        None,
    )
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Development",
        "dev",
        development_config_genesis,
        vec![],
        None,
        None,
        None,
        None,
    )
}

fn local_testnet_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![
            get_authority_keys_from_seed("Alice"),
            get_authority_keys_from_seed("Bob"),
        ],
        get_account_id_from_seed("Alice").into(),
        None,
    )
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Local Testnet",
        "local_testnet",
        local_testnet_genesis,
        vec![],
        // TODO, remove it when substrate upgrade to latest version. test that hasn't this problem.
        Some(TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])),
        None,
        None,
        None,
    )
}

fn abmatrix_testnet_genesis() -> GenesisConfig {
    let alice_stash   = ed25519::Public::from_ss58check("5GoKvZWG5ZPYL1WUovuHW3zJBWBP5eT8CbqjdRY4Q6iMaDtZ").unwrap().0;
    let alice_control = ed25519::Public::from_ss58check("5GoKvZWG5ZPYL1WUovuHW3zJBWBP5eT8CbqjdRY4Q6iMaDtZ").unwrap().0;
    let bob_stash   = ed25519::Public::from_ss58check("5Gw3s7q4QLkSWwknsiPtjujPv3XM4Trxi5d4PgKMMk3gfGTE").unwrap().0;
    let bob_control = ed25519::Public::from_ss58check("5Gw3s7q4QLkSWwknsiPtjujPv3XM4Trxi5d4PgKMMk3gfGTE").unwrap().0;
    let eve_stash   = ed25519::Public::from_ss58check("5CNLHq4doqBbrrxLCxAakEgaEvef5tjSrN7QqJwcWzNd7W7k").unwrap().0;
    let eve_control = ed25519::Public::from_ss58check("5CNLHq4doqBbrrxLCxAakEgaEvef5tjSrN7QqJwcWzNd7W7k").unwrap().0;

    testnet_genesis(
        vec![
            (alice_stash.into(), alice_control.into(), alice_control.into()),
            (bob_stash.into(), bob_control.into(), bob_control.into()),
            (eve_stash.into(), eve_control.into(), eve_control.into()),
        ],
        alice_control.into(),
        Some(vec![alice_control.into(), bob_control.into(), eve_control.into()]),
    )
}

/// abmatrix testnet config
pub fn abmatrix_testnet_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Abmatrix Testnet",
        "abmatrix_testnet",
        local_testnet_genesis,
        vec![],
        // TODO, remove it when substrate upgrade to latest version. test that hasn't this problem.
        Some(TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])),
        None,
        None,
        None,
    )
}