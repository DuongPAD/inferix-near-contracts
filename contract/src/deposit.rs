use crate::account_info::AccountInfo;
use crate::Contract;
use crate::ContractExt;

use near_sdk::assert_one_yocto;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::Serialize;
use near_sdk::{env, log, near_bindgen, AccountId, Balance, Promise};
pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;
pub const DECIMALS_POW: u128 = 1_000_000_000_000;
pub const MIN_DEPOSIT_COST: u128 = 5_000_000_000_000_000_000_000_000;

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
        self.abort_if_pause();
        self.abort_if_blacklisted(sender.clone());

        if attached_deposit < MIN_DEPOSIT_COST {
            env::panic_str("The attached deposit must be at least 5 NEAR");
        }

        // let mut deposited_so_far = self.deposits.get(&sender).unwrap_or(0);
        let mut deposited_so_far = self.deposits.get(&sender).unwrap_or_default();

        let to_transfer: Balance = if deposited_so_far.deposited == 0 {
            // Subtract the storage cost to the amount to transfer
            attached_deposit - STORAGE_COST
        } else {
            attached_deposit
        };

        // Persist in storage the amount deposited so far
        deposited_so_far.add_deposited(to_transfer);
        // deposited_so_far += attached_deposit;

        self.deposits.insert(&sender, &deposited_so_far);

        log!(
            "Thank you {} for depositing {}! You deposited a total of {}",
            sender.clone(),
            attached_deposit,
            deposited_so_far.deposited
        );

        // Send the money to the vault
        Promise::new(self.vault.clone()).transfer(to_transfer);

        // Return the total amount deposited so far
        U128(deposited_so_far.deposited)
    }

    #[payable]
    pub fn withdraw_all(&mut self) {
        //make sure the user attaches exactly 1 yoctoNEAR for security purposes.
        //this will redirect them to the NEAR wallet (or requires a full access key).
        assert_one_yocto();

        let sender = env::predecessor_account_id();

        let mut user_info = self.deposits.get(&sender).unwrap_or_default();
        self.abort_if_not_in_withdraw_time(&user_info);
        let amount_remained = user_info.get_remained();

        //if that excess to withdraw is > 0, we transfer the amount to the user.
        if amount_remained > 0 {
            Promise::new(sender.clone()).transfer(amount_remained);
        }
        self.deposits.insert(&sender, &AccountInfo::default());

        log!(
            "Thank you {} for withdrawing {}! ",
            sender.clone(),
            amount_remained,
        );
    }

    #[payable]
    pub fn withdraw_a_part(&mut self, amount: Balance) {
        //make sure the user attaches exactly 1 yoctoNEAR for security purposes.
        //this will redirect them to the NEAR wallet (or requires a full access key).
        assert_one_yocto();

        let sender = env::predecessor_account_id();

        let mut user_info = self.deposits.get(&sender).unwrap_or_default();
        self.abort_if_not_in_withdraw_time(&user_info);

        let real_amount = amount * DECIMALS_POW;

        assert!(user_info.get_remained() > real_amount, "Withdraw too much");

        Promise::new(sender.clone()).transfer(real_amount);
        user_info.withdraw(real_amount);

        self.deposits.insert(&sender, &user_info);

        log!("Thank you {} for withdrawing {}! ", sender.clone(), amount,);
    }

    // Public - get deposit for account ID
    pub fn get_deposit_for_account(&self, account_id: AccountId) -> Deposit {
        Deposit {
            account_id: account_id.clone(),
            total_amount: U128(self.deposits.get(&account_id).unwrap_or_default().deposited),
        }
    }

    // Public - get info by account ID
    pub fn get_info_for_account(&self, account_id: AccountId) -> AccountInfo {
        let deposit_by_id = self.deposits.get(&account_id).unwrap_or_default();
        AccountInfo {
            deposited: deposit_by_id.clone().deposited,
            spent: deposit_by_id.clone().spent,
            remained: deposit_by_id.clone().remained,
            start_at: deposit_by_id.clone().start_at,
            end_at: deposit_by_id.clone().end_at,
        }
    }

    // Public - get total number of users
    pub fn get_number_of_users(&self) -> u64 {
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

    // Private - Governance set spent for account
    #[payable]
    pub fn set_user_spent(&mut self, account_id: AccountId, amount: Balance) {
        assert_one_yocto();
        self.abort_if_not_governance();
        self.abort_if_pause();
        let real_amount = amount * DECIMALS_POW;

        let mut user_info = self.deposits.get(&account_id.clone()).unwrap_or_default();
        assert!(user_info.deposited > real_amount, "Spent too much");
        user_info.set_spent(real_amount);
        let now_sec = env::block_timestamp_ms() / 1000;
        user_info.set_time(now_sec, now_sec + self.get_allowance_time());
        self.deposits.insert(&account_id.clone(), &user_info);
    }
}
