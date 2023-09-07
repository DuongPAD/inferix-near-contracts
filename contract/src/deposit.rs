use crate::Contract;
use crate::ContractExt;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::Serialize;
use near_sdk::{env, log, near_bindgen, AccountId, Balance, Promise};

pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Deposit {
    pub account_id: AccountId,
    pub total_amount: U128,
}

#[near_bindgen]
impl Contract {
    #[payable] // Public - People can attach money
    pub fn deposit(&mut self) -> U128 {
        // Get who is calling the method and how much $NEAR they attached
        let attached_deposit: Balance = env::attached_deposit();
        let sender: AccountId = env::predecessor_account_id();

        let mut deposited_so_far = self.deposits.get(&sender).unwrap_or(0);

        let to_transfer: Balance = if deposited_so_far == 0 {
            // This is the user's first deposit, lets register it, which increases storage
            assert!(
                attached_deposit > STORAGE_COST,
                "Attach at least {} yoctoNEAR",
                STORAGE_COST
            );

            // Subtract the storage cost to the amount to transfer
            attached_deposit - STORAGE_COST
        } else {
            attached_deposit
        };

        // Persist in storage the amount deposited so far
        deposited_so_far += attached_deposit;
        self.deposits.insert(&sender, &deposited_so_far);

        log!(
            "Thank you {} for depositing {}! You deposited a total of {}",
            sender.clone(),
            attached_deposit,
            deposited_so_far
        );

        // Send the money to the vault
        Promise::new(self.vault.clone()).transfer(to_transfer);

        // Return the total amount deposited so far
        U128(deposited_so_far)
    }

    // Public - get deposit by account ID
    pub fn get_deposit_for_account(&self, account_id: AccountId) -> Deposit {
        Deposit {
            account_id: account_id.clone(),
            total_amount: U128(self.deposits.get(&account_id).unwrap_or(0)),
        }
    }

    // Public - get total number of users
    pub fn number_of_users(&self) -> u64 {
        self.deposits.len()
    }

    // Public - paginate through all deposits on the contract
    pub fn get_deposits(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Deposit> {
        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through deposit
        self.deposits
            .keys()
            //skip to the index we specified in the start variable
            .skip(start as usize)
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize)
            .map(|account| self.get_deposit_for_account(account))
            //since we turned map into an iterator, we need to turn it back into a vector to return
            .collect()
    }
}
