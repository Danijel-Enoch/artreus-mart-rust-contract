use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn withdraw_mint_fee(&self) -> bool {
        //require that it is owner
        assert!(
            self.owner_id == env::current_account_id(),
            "Only owner can cash out"
        );
        assert!(
            self.owner_id == env::predecessor_account_id(),
            "Only owner can cash out (pred)"
        );
        //get account balance
        let account_balance: Balance = env::account_balance();
        let to = env::current_account_id();
        //require that the balance is less than zero
        assert!(account_balance > 0, "Balance too low for withdrawal");

        //then transfer to owner
        Promise::new(to).transfer(account_balance);

        true
    }
}
