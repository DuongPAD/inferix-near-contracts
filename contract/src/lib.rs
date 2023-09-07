use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId};

mod account_info;
mod deposit;
mod governance;
use crate::account_info::AccountInfo;

#[derive(BorshDeserialize, BorshSerialize, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum BlackListStatus {
    // An address might be using
    Allowable,
    // All acts with an address have to be banned
    Banned,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum ContractStatus {
    Working,
    Paused,
}

impl std::fmt::Display for ContractStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContractStatus::Working => write!(f, "working"),
            ContractStatus::Paused => write!(f, "paused"),
        }
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    vault: AccountId,
    black_list: LookupMap<AccountId, BlackListStatus>,
    deposits: UnorderedMap<AccountId, AccountInfo>,
    status: ContractStatus,
    governance: AccountId,
    withdraw_allowance_time: u64,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            vault: "skywalker99.testnet".parse().unwrap(),
            black_list: LookupMap::new(b"b"),
            deposits: UnorderedMap::new(b"d"),
            status: ContractStatus::Working,
            governance: "skywalker99.testnet".parse().unwrap(),
            withdraw_allowance_time: 300,
        }
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    #[private] // Public - but only callable by env::current_account_id()
    pub fn init(governance: AccountId) -> Self {
        Self {
            vault: governance.clone(),
            black_list: LookupMap::new(b"b"),
            deposits: UnorderedMap::new(b"d"),
            status: ContractStatus::Working,
            governance,
            withdraw_allowance_time: 300,
        }
    }

    // Public - contract status getter
    pub fn contract_status(&self) -> ContractStatus {
        self.status.clone()
    }

    pub fn blacklist_status(&self, account_id: &AccountId) -> BlackListStatus {
        return match self.black_list.get(account_id) {
            Some(x) => x.clone(),
            None => BlackListStatus::Allowable,
        };
    }

    fn abort_if_pause(&self) {
        if self.status == ContractStatus::Paused {
            env::panic_str("The contract is under maintenance")
        }
    }

    fn abort_if_not_governance(&self) {
        let sender: AccountId = env::predecessor_account_id();
        if self.governance != sender {
            env::panic_str("Only governance")
        }
    }
    fn abort_if_not_in_withdraw_time(&self, user_info: &AccountInfo) {
        let now_sec = env::block_timestamp();
        if user_info.start_at > now_sec || user_info.end_at < now_sec {
            env::panic_str("Not in withdraw time")
        }
    }

    fn abort_if_blacklisted(&self, account_id: AccountId) {
        if self.blacklist_status(&account_id) != BlackListStatus::Allowable {
            env::panic_str(&format!("Account '{}' is banned", account_id));
        }
    }

    // Public - vault getter
    pub fn get_vault(&self) -> AccountId {
        self.vault.clone()
    }
    // Public - withdraw_allowance_time
    pub fn get_allowance_time(&self) -> u64 {
        self.withdraw_allowance_time.clone()
    }

    // Public - vault getter
    pub fn get_all_deposits(&self) -> Vec<(AccountId, AccountInfo)> {
        self.deposits.iter().collect()
    }

    // Public - but only callable by env::current_account_id(). Sets the vault
    #[private]
    pub fn change_vault(&mut self, vault: AccountId) {
        self.abort_if_not_governance();
        self.vault = vault;
    }
    pub fn change_allowance_time(&mut self, time: u64) {
        self.abort_if_not_governance();
        self.withdraw_allowance_time = time;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;
    use near_sdk::Balance;

    const VAULT: &str = "vault";
    const NEAR: u128 = 1000000000000000000000000;

    #[test]
    fn initializes() {
        let contract = Contract::init(VAULT.parse().unwrap());
        assert_eq!(contract.vault, VAULT.parse().unwrap())
    }

    #[test]
    fn deposit() {
        let mut contract = Contract::init(VAULT.parse().unwrap());

        // Make a deposit
        set_context("user_a", 1 * NEAR);
        contract.deposit();
        let first_deposit = contract.get_deposit_for_account("user_a".parse().unwrap());

        // Check the deposit was recorded correctly
        assert_eq!(first_deposit.total_amount.0, 1 * NEAR);

        // Make another deposit
        set_context("user_b", 2 * NEAR);
        contract.deposit();
        let second_deposit = contract.get_deposit_for_account("user_b".parse().unwrap());

        // Check the deposit was recorded correctly
        assert_eq!(second_deposit.total_amount.0, 2 * NEAR);

        // User A makes another deposit on top of their original
        set_context("user_a", 1 * NEAR);
        contract.deposit();
        let first_deposit = contract.get_deposit_for_account("user_a".parse().unwrap());

        // Check the deposit was recorded correctly
        assert_eq!(first_deposit.total_amount.0, 1 * NEAR * 2);

        assert_eq!(contract.number_of_users(), 2);
    }

    // Auxiliar fn: create a mock context
    fn set_context(predecessor: &str, amount: Balance) {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor.parse().unwrap());
        builder.attached_deposit(amount);

        testing_env!(builder.build());
    }
}
