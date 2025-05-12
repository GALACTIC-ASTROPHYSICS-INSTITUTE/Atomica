// example_token.rs

use ink_lang as ink;

#[ink::contract]
mod example_token {
    #[ink(storage)]
    pub struct ExampleToken {
        total_supply: Balance,
        balances: ink_storage::collections::HashMap<AccountId, Balance>,
    }

    impl ExampleToken {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let mut balances = ink_storage::collections::HashMap::new();
            let caller = Self::env().caller();
            balances.insert(caller, initial_supply);
            Self {
                total_supply: initial_supply,
                balances,
            }
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            *self.balances.get(&owner).unwrap_or(&0)
        }
    }
}
