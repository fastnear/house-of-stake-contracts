use crate::venear_ext::{ext_venear, GAS_FOR_VENEAR_LOCKUP_UPDATE};
use crate::*;
use common::lockup_update::{LockupUpdateV1, VLockupUpdate};
use near_sdk::json_types::U64;
use near_sdk::{assert_one_yocto, log, near, NearToken};

impl LockupContract {
    fn storage_usage(&self) -> NearToken {
        NearToken::from_yoctonear(MIN_BALANCE_FOR_STORAGE)
        // env::storage_byte_cost()
        //     .checked_mul(env::storage_usage() as u128)
        //     .unwrap()
    }

    pub(crate) fn venear_liquid_balance(&self) -> Balance {
        let remaining_balance = env::account_balance();
        //            .checked_sub(self.storage_usage())
        //            .unwrap();

        remaining_balance
            .checked_sub(NearToken::from_yoctonear(self.venear_locked_balance))
            .expect("Illegal balance")
            .checked_sub(NearToken::from_yoctonear(self.venear_pending_balance))
            .expect("Illegal balance")
            .as_yoctonear()
    }

    fn set_venear_unlock_imestamp(&mut self) {
        self.venear_unlock_timestamp = env::block_timestamp() + self.unlock_duration_ns;
    }

    fn venear_lockup_update(&mut self) {
        self.lockup_update_nonce += 1;

        // Calls veNEAR with new total NEAR balance locked in the lockup
        ext_venear::ext(self.venear_account_id.clone())
            .with_static_gas(GAS_FOR_VENEAR_LOCKUP_UPDATE)
            .on_lockup_update(
                self.version,
                self.owner_account_id.clone(),
                VLockupUpdate::V1(LockupUpdateV1 {
                    locked_near_balance: NearToken::from_yoctonear(self.venear_locked_balance),
                    timestamp: env::block_timestamp().into(),
                    lockup_update_nonce: U64::from(self.lockup_update_nonce),
                }),
            );
    }
}

#[near]
impl LockupContract {
    pub fn get_venear_locked_balance(&self) -> WrappedBalance {
        self.venear_locked_balance.into()
    }

    pub fn get_venear_unlock_timestamp(&self) -> Timestamp {
        self.venear_unlock_timestamp
    }

    pub fn get_lockup_update_nonce(&self) -> u64 {
        self.lockup_update_nonce
    }

    pub fn get_venear_pending_balance(&self) -> WrappedBalance {
        self.venear_pending_balance.into()
    }

    pub fn get_venear_liquid_balance(&self) -> WrappedBalance {
        self.venear_liquid_balance().into()
    }

    /// specify the amount of near you want to lock, it remembers how much near is now locked
    #[payable]
    pub fn lock_near(&mut self, amount: Option<WrappedBalance>) {
        assert_one_yocto();
        let amount: Balance = if let Some(amount) = amount {
            amount.into()
        } else {
            self.venear_liquid_balance()
        };

        assert!(amount <= self.venear_liquid_balance(), "Invalid amount");

        self.venear_locked_balance += amount;

        self.venear_lockup_update();
    }

    /// you specify the amount of near to unlock, it starts the process of unlocking it
    /// (works similarly to unstaking from a staking pool).
    #[payable]
    pub fn begin_unlock_near(&mut self, amount: Option<WrappedBalance>) {
        assert_one_yocto();
        let amount: Balance = if let Some(amount) = amount {
            amount.into()
        } else {
            self.venear_locked_balance
        };

        assert!(
            amount <= self.venear_locked_balance,
            "Invalid amount"
        );

        self.venear_locked_balance -= amount;
        self.venear_pending_balance += amount;
        self.set_venear_unlock_imestamp();

        self.venear_lockup_update();
    }

    /// end the unlocking
    #[payable]
    pub fn end_unlock_near(&mut self, amount: Option<WrappedBalance>) {
        assert_one_yocto();
        let amount: Balance = if let Some(amount) = amount {
            amount.into()
        } else {
            self.venear_pending_balance
        };

        assert!(amount <= self.venear_pending_balance, "Invalid amount");
        assert!(
            env::block_timestamp() >= self.venear_unlock_timestamp,
            "Invalid unlock time"
        );

        self.venear_pending_balance -= amount;

        self.venear_lockup_update();
    }

    ///  if there is an unlock pending, it locks the balance.
    #[payable]
    pub fn lock_pending_near(&mut self, amount: Option<WrappedBalance>) {
        assert_one_yocto();
        let amount: Balance = if let Some(amount) = amount {
            amount.into()
        } else {
            self.venear_pending_balance
        };

        assert!(amount <= self.venear_pending_balance, "Invalid amount");

        self.venear_pending_balance -= amount;
        self.venear_locked_balance += amount;

        self.venear_lockup_update();
    }
}
