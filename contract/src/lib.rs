use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId};

mod deposit;
mod governance;

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
    deposits: UnorderedMap<AccountId, u128>,
    status: ContractStatus,
    governance: AccountId,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            vault: "skywalker99.testnet".parse().unwrap(),
            deposits: UnorderedMap::new(b"d"),
            status: ContractStatus::Working,
            governance: "skywalker99.testnet".parse().unwrap(),
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
            deposits: UnorderedMap::new(b"d"),
            status: ContractStatus::Working,
            governance,
        }
    }

    // Public - contract status getter
    pub fn contract_status(&self) -> ContractStatus {
        self.status.clone()
    }

    fn abort_if_pause(&self) {
        if self.status == ContractStatus::Paused {
            env::panic_str("The contract is under maintenance")
        }
    }

    // Public - vault getter
    pub fn get_vault(&self) -> AccountId {
        self.vault.clone()
    }

    // Public - vault getter
    pub fn get_all_deposits(&self) -> Vec<(AccountId, u128)> {
        self.deposits.iter().collect()
    }

    // Public - but only callable by env::current_account_id(). Sets the vault
    #[private]
    pub fn change_vault(&mut self, vault: AccountId) {
        self.vault = vault;
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
