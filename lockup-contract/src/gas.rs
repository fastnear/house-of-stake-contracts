use crate::*;

const BASE_GAS: Gas = Gas::from_gas(25_000_000_000_000);

pub mod whitelist {

    use near_sdk::Gas;
    /// Gas attached to the promise to check whether the given staking pool Account ID is
    /// whitelisted.
    /// Requires BASE (no external calls).
    pub const IS_WHITELISTED: Gas = super::BASE_GAS;
}

pub mod staking_pool {
    use near_sdk::Gas;

    /// Gas attached to deposit call on the staking pool contract.
    /// Requires BASE for local updates + BASE potentially restake.
    pub const DEPOSIT: Gas = Gas::from_gas(super::BASE_GAS.as_gas() * 2);

    /// Gas attached to deposit call on the staking pool contract.
    /// Requires BASE for local updates + 2 * BASE for staking call.
    pub const DEPOSIT_AND_STAKE: Gas = Gas::from_gas(super::BASE_GAS.as_gas() * 3);

    /// Gas attached to withdraw call on the staking pool contract.
    /// Requires BASE for execution + 2 * BASE for transferring amount to us and potentially restake.
    pub const WITHDRAW: Gas = Gas::from_gas(super::BASE_GAS.as_gas() * 3);

    /// Gas attached to stake call on the staking pool contract.
    /// Requires BASE for execution + 2 * BASE for staking call.
    pub const STAKE: Gas = Gas::from_gas(super::BASE_GAS.as_gas() * 3);

    /// Gas attached to unstake call on the staking pool contract.
    /// Requires BASE for execution + 2 * BASE for staking call.
    pub const UNSTAKE: Gas = Gas::from_gas(super::BASE_GAS.as_gas() * 3);

    /// Gas attached to unstake all call on the staking pool contract.
    /// Requires BASE for execution + 2 * BASE for staking call.
    pub const UNSTAKE_ALL: Gas = Gas::from_gas(super::BASE_GAS.as_gas() * 3);

    /// The amount of gas required to get the current staked balance of this account from the
    /// staking pool.
    /// Requires BASE for local processing.
    pub const GET_ACCOUNT_STAKED_BALANCE: Gas = super::BASE_GAS;

    /// The amount of gas required to get current unstaked balance of this account from the
    /// staking pool.
    /// Requires BASE for local processing.
    pub const GET_ACCOUNT_UNSTAKED_BALANCE: Gas = super::BASE_GAS;

    /// The amount of gas required to get the current total balance of this account from the
    /// staking pool.
    /// Requires BASE for local processing.
    pub const GET_ACCOUNT_TOTAL_BALANCE: Gas = super::BASE_GAS;
}

// TODO: Deprecate
pub mod transfer_poll {
    use near_sdk::Gas;

    /// Gas attached to the promise to check whether transfers were enabled on the transfer poll
    /// contract.
    /// Requires BASE (no external calls).
    pub const GET_RESULT: Gas = super::BASE_GAS;
}

pub mod owner_callbacks {
    use near_sdk::Gas;

    /// Gas attached to the inner callback for processing whitelist check results.
    /// Requires BASE for local execution.
    pub const ON_WHITELIST_IS_WHITELISTED: Gas = super::BASE_GAS;

    /// Gas attached to the inner callback for processing result of the deposit call to the
    /// staking pool.
    /// Requires BASE for local updates.
    pub const ON_STAKING_POOL_DEPOSIT: Gas = super::BASE_GAS;

    /// Gas attached to the inner callback for processing result of the deposit and stake call to
    /// the staking pool.
    /// Requires BASE for local updates.
    pub const ON_STAKING_POOL_DEPOSIT_AND_STAKE: Gas = super::BASE_GAS;

    /// Gas attached to the inner callback for processing result of the withdraw call to the
    /// staking pool.
    /// Requires BASE for local updates.
    pub const ON_STAKING_POOL_WITHDRAW: Gas = super::BASE_GAS;

    /// Gas attached to the inner callback for processing result of the stake call to the
    /// staking pool.
    pub const ON_STAKING_POOL_STAKE: Gas = super::BASE_GAS;

    /// Gas attached to the inner callback for processing result of the unstake call to the
    /// staking pool.
    /// Requires BASE for local updates.
    pub const ON_STAKING_POOL_UNSTAKE: Gas = super::BASE_GAS;

    /// Gas attached to the inner callback for processing result of the unstake all call to the
    /// staking pool.
    /// Requires BASE for local updates.
    pub const ON_STAKING_POOL_UNSTAKE_ALL: Gas = super::BASE_GAS;

    /// Gas attached to the inner callback for processing result of the checking result for
    /// transfer voting call to the voting contract.
    /// Requires BASE for local updates.
    pub const ON_VOTING_GET_RESULT: Gas = super::BASE_GAS;

    /// Gas attached to the inner callback for processing result of the call to get the current
    /// total balance from the staking pool.
    /// Requires BASE for local updates.
    pub const ON_GET_ACCOUNT_TOTAL_BALANCE: Gas = super::BASE_GAS;

    /// Gas attached to the inner callback for processing result of the call to get the current
    /// unstaked balance from the staking pool.
    /// The callback might proceed with withdrawing this amount.
    /// Requires BASE for local updates + gas for withdraw + gas for another callback.
    pub const ON_GET_ACCOUNT_UNSTAKED_BALANCE_TO_WITHDRAW_BY_OWNER: Gas = Gas::from_gas(
        super::BASE_GAS.as_gas()
            + super::staking_pool::WITHDRAW.as_gas()
            + ON_STAKING_POOL_WITHDRAW.as_gas(),
    );
}
