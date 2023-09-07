use near_sdk::assert_one_yocto;

use crate::*;

#[near_bindgen]
impl Contract {
    pub(crate) fn assert_governance(&self) {
        if env::predecessor_account_id() != self.governance {
            env::panic_str("This method can be called only by governance")
        }
    }

    pub fn set_governance(&mut self, governance: AccountId) {
        self.assert_governance();
        self.governance = governance;
    }

    pub fn governance_id(&self) -> AccountId {
        self.governance.clone()
    }

    pub fn add_to_blacklist(&mut self, account_id: &AccountId) {
        self.assert_governance();
        self.black_list.insert(account_id, &BlackListStatus::Banned);
    }

    pub fn remove_from_blacklist(&mut self, account_id: &AccountId) {
        self.assert_governance();
        self.black_list
            .insert(account_id, &BlackListStatus::Allowable);
    }

    /// Pauses the contract. Only can be called by owner or guardians.
    #[payable]
    pub fn pause(&mut self) {
        assert_one_yocto();
        // TODO: Should guardians be able to pause?
        self.assert_governance();
        self.status = ContractStatus::Paused;
    }

    /// Resumes the contract. Only can be called by owner.
    pub fn resume(&mut self) {
        self.assert_governance();
        self.status = ContractStatus::Working;
    }
}
