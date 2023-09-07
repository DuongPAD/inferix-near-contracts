use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, Balance};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountInfo {
    pub user: AccountId,
    pub deposited: Balance,
    pub spent: Balance,
    pub remained: Balance,
    pub start_at: u64,
    pub end_at: u64,
}
impl Default for AccountInfo {
    fn default() -> AccountInfo {
        AccountInfo {
            user: "skywalker99.testnet".parse().unwrap(),
            deposited: 0,
            spent: 0,
            remained: 0,
            start_at: 0,
            end_at: 0,
        }
    }
}

impl AccountInfo {
    /// Adds amount to the balance of given token
    pub(crate) fn add_deposited(&mut self, amount: Balance) {
        self.deposited += amount;
        self.remained += amount;
    }
    pub(crate) fn withdraw(&mut self, amount: Balance) {
        assert!(self.deposited >= amount, "overflow");

        self.deposited -= amount;
        self.remained -= amount;
    }
    pub(crate) fn set_spent(&mut self, amount: Balance) {
        assert!(self.deposited >= amount, "overflow");
        self.spent = amount;
        self.remained = self.deposited - amount;
    }
    pub(crate) fn set_time(&mut self, start: u64, end: u64) {
        assert!(end > start, "overflow");
        self.start_at = start;
        self.end_at = end;
    }
    pub(crate) fn get_deposited(&mut self) -> Balance {
        self.deposited
    }
    pub(crate) fn get_spent(&mut self) -> Balance {
        self.spent
    }
    pub(crate) fn get_remained(&mut self) -> Balance {
        self.remained
    }
    pub(crate) fn get_withdraw_time(&mut self) -> (u64, u64) {
        (self.start_at, self.end_at)
    }
}
