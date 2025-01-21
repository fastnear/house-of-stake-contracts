use crate::*;
use near_sdk::json_types::{U128, U64};
use uint::construct_uint;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

/// Raw type for balance in yocto NEAR.
pub type Balance = u128;

/// Raw type for duration in nanoseconds
pub type Duration = u64;
/// Raw type for timestamp in nanoseconds
pub type Timestamp = u64;

/// Timestamp in nanosecond wrapped into a struct for JSON serialization as a string.
pub type WrappedTimestamp = U64;
/// Duration in nanosecond wrapped into a struct for JSON serialization as a string.
pub type WrappedDuration = U64;
/// Balance wrapped into a struct for JSON serialization as a string.
pub type WrappedBalance = U128;

/// Contains information about token lockups.
#[near(serializers=[borsh])]
pub struct LockupInformation {
    /// The amount in yocto-NEAR tokens locked for this account.
    pub lockup_amount: Balance,
    /// [deprecated] - the duration in nanoseconds of the lockup period from
    /// the moment the transfers are enabled. During this period tokens are locked and
    /// the release doesn't start. Instead of this, use `lockup_timestamp` and `release_duration`
    pub lockup_duration: Duration,
    /// If present, it is the duration when the full lockup amount will be available. The tokens
    /// are linearly released from the moment tokens are unlocked, defined by:
    /// `max(transfers_timestamp + lockup_duration, lockup_timestamp)`.
    /// If not present, the tokens are not locked (though, vesting logic could be used).
    pub release_duration: Option<Duration>,
    /// The optional absolute lockup timestamp in nanoseconds which locks the tokens until this
    /// timestamp passes. Until this moment the tokens are locked and the release doesn't start.
    /// If not present, `transfers_timestamp` will be used.
    pub lockup_timestamp: Option<Timestamp>,
    /// The information about the transfers. Either transfers are already enabled, then it contains
    /// the timestamp when they were enabled. Or the transfers are currently disabled and
    /// it contains the account ID of the transfer poll contract.
    pub transfers_information: TransfersInformation,
}

/// Contains information about the transfers. Whether transfers are enabled or disabled.
#[near(serializers=[borsh, json])]
pub enum TransfersInformation {
    /// The timestamp when the transfers were enabled.
    TransfersEnabled {
        transfers_timestamp: WrappedTimestamp,
    },
    /// The account ID of the transfers poll contract, to check if the transfers are enabled.
    /// The lockup period can start only after the transfer voted to be enabled.
    /// At the launch of the network transfers are disabled for all lockup contracts, once transfers
    /// are enabled, they can't be disabled and don't need to be checked again.
    TransfersDisabled { transfer_poll_account_id: AccountId },
}

/// Describes the status of transactions with the staking pool contract or terminated unvesting
/// amount withdrawal.
#[near(serializers=[borsh, json])]
pub enum TransactionStatus {
    /// There are no transactions in progress.
    Idle,
    /// There is a transaction in progress.
    Busy,
}

/// Contains information about current stake and delegation.
#[near(serializers=[borsh])]
pub struct StakingInformation {
    /// The Account ID of the staking pool contract.
    pub staking_pool_account_id: AccountId,

    /// Contains status whether there is a transaction in progress.
    pub status: TransactionStatus,

    /// The amount of tokens that were deposited from this account to the staking pool.
    /// NOTE: The unstaked amount on the staking pool might be higher due to staking rewards.
    pub deposit_amount: WrappedBalance,
}


/// The result of the transfer poll.
/// Contains The timestamp when the proposal was voted in.
pub type PollResult = Option<WrappedTimestamp>;
