use crate::*;
use near_sdk::assert_one_yocto;
use near_sdk::json_types::{Base58CryptoHash, U64};

#[near]
impl Contract {
    /// Updates the active lockup contract to the given contract hash and sets the minimum lockup
    /// deposit.
    /// Can only be called by the owner.
    /// Requires 1 yocto NEAR.
    #[payable]
    pub fn set_lockup_contract(
        &mut self,
        contract_hash: Base58CryptoHash,
        min_lockup_deposit: NearToken,
    ) {
        assert_one_yocto();
        self.assert_owner();
        self.internal_set_lockup(contract_hash.into());
        self.config.min_lockup_deposit = min_lockup_deposit;
    }

    /// Sets the amount in NEAR required for local storage in veNEAR contract.
    /// Can only be called by the owner.
    /// Requires 1 yocto NEAR.
    #[payable]
    pub fn set_local_deposit(&mut self, local_deposit: NearToken) {
        assert_one_yocto();
        self.assert_owner();
        self.config.local_deposit = local_deposit;
    }

    /// Sets the account ID of the staking pool whitelist for lockup contract.
    /// Can only be called by the owner.
    /// Requires 1 yocto NEAR.
    #[payable]
    pub fn set_staking_pool_whitelist_account_id(
        &mut self,
        staking_pool_whitelist_account_id: AccountId,
    ) {
        assert_one_yocto();
        self.assert_owner();
        self.config.staking_pool_whitelist_account_id = staking_pool_whitelist_account_id;
    }

    /// Proposes the new owner account ID.
    /// Can only be called by the owner.
    /// Requires 1 yocto NEAR.
    #[payable]
    pub fn propose_new_owner_account_id(&mut self, new_owner_account_id: Option<AccountId>) {
        assert_one_yocto();
        self.assert_owner();
        self.config.proposed_new_owner_account_id = new_owner_account_id;
    }

    /// Accepts the new owner account ID.
    /// Can only be called by the new owner.
    /// Requires 1 yocto NEAR.
    #[payable]
    pub fn accept_ownership(&mut self) {
        assert_one_yocto();
        let predecessor = env::predecessor_account_id();
        require!(
            self.config.proposed_new_owner_account_id.as_ref() == Some(&predecessor),
            "Only the proposed new owner can call this method"
        );
        self.config.owner_account_id = predecessor;
        self.config.proposed_new_owner_account_id = None;
    }

    /// Sets the unlock duration in seconds.
    /// Note, this method will only affect new lockups.
    /// Can only be called by the owner.
    /// Requires 1 yocto NEAR.
    #[payable]
    pub fn set_unlock_duration_sec(&mut self, unlock_duration_sec: u32) {
        assert_one_yocto();
        self.assert_owner();
        self.config.unlock_duration_ns = U64::from(unlock_duration_sec as u64 * 1_000_000_000);
    }

    /// Sets the list of account IDs that can store new lockup contract code.
    /// Can only be called by the owner.
    /// Requires 1 yocto NEAR.
    #[payable]
    pub fn set_lockup_code_deployers(&mut self, lockup_code_deployers: Vec<AccountId>) {
        assert_one_yocto();
        self.assert_owner();
        self.config.lockup_code_deployers = lockup_code_deployers;
    }

    /// Sets the list of account IDs that can pause the contract.
    /// Can only be called by the owner.
    /// Requires 1 yocto NEAR.
    #[payable]
    pub fn set_guardians(&mut self, guardians: Vec<AccountId>) {
        assert_one_yocto();
        self.assert_owner();
        self.config.guardians = guardians;
    }
}

impl Contract {
    pub fn assert_owner(&self) {
        require!(
            env::predecessor_account_id() == self.config.owner_account_id,
            "Only the owner can call this method"
        );
    }

    /// Asserts that the caller is one of the guardians or the owner.
    pub fn assert_guardian(&self) {
        let predecessor = env::predecessor_account_id();
        require!(
            self.config.guardians.contains(&predecessor)
                || predecessor == self.config.owner_account_id,
            "Only the guardian can call this method"
        );
    }
}
