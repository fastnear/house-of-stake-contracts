mod account;
mod config;
mod delegation;
mod global_state;
mod lockup;
mod snapshot;

use merkle_tree::{MerkleProof, MerkleTree, MerkleTreeSnapshot};

use crate::account::VAccountInternal;
use crate::config::Config;
use common::account::*;
use common::global_state::*;
use common::venear::VenearGrowthConfig;
use common::Version;
use near_sdk::store::LookupMap;
use near_sdk::{
    env, near, require, sys, AccountId, BorshStorageKey, CryptoHash, NearToken, PanicOnDefault,
};

#[derive(BorshStorageKey)]
#[near]
enum StorageKeys {
    Tree,
    LockupCode(CryptoHash),
    Accounts,
    Lsts,
}

#[derive(PanicOnDefault)]
#[near(contract_state)]
pub struct Contract {
    tree: MerkleTree<VAccount, VGlobalState>,
    accounts: LookupMap<AccountId, VAccountInternal>,
    config: Config,
}

#[near]
impl Contract {
    #[init]
    pub fn init(config: Config, venear_growth_config: VenearGrowthConfig) -> Self {
        Self {
            tree: MerkleTree::new(
                StorageKeys::Tree,
                GlobalState::new(env::block_timestamp().into(), venear_growth_config).into(),
            ),
            accounts: LookupMap::new(StorageKeys::Accounts),
            config,
        }
    }

    //TODO
    // Flow
    // 1. Check if account exists (get_account_info)
    // 2. If not, create new account (attempts to deploys a new lockup. Keeps some funds for local storage)
    //
    // veNEAR implementation:
    // - Get veNear balance
    //
    // Internal:
    // - update veNear from lockup, e.g. unlocking, relocking.
    // - internal configuration
    // - contract upgrades
    //
    // Making snapshots for new voting
    //
    // Delegation of veNEAR
    //
    // Voting
    // Lockup integration on account update

    // TODO: delegations
    // TODO: veNEAR token non-transferable implementation
}
