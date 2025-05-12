#![cfg_attr(not(feature = "std"), no_std)]

pub use ink::prelude::vec::Vec;
use ink::storage::Mapping;
use ink::contract::ContractEnv;
use ink::env::DefaultEnvironment;
use ink::storage::traits::SpreadAllocate;
use ink::contract::Contract;

#[ink::contract]
mod token {
    use super::*;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Token {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>,
        name: Vec<u8>,
        symbol: Vec<u8>,
        decimals: u8,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    impl Token {
        #[ink(constructor)]
        pub fn new(name: Vec<u8>, symbol: Vec<u8>, decimals: u8) -> Self {
            let caller = Self::env().caller();
            let mut instance = Self::allocate();
            instance.name = name;
            instance.symbol = symbol;
            instance.decimals = decimals;
            instance.total_supply = 0;
            instance.balances.insert(caller, &0);
            instance
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap_or(0)
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<(), &'static str> {
            let from = self.env().caller();
            self._transfer(from, to, value)
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), &'static str> {
            let owner = self.env().caller();
            self.allowances.insert((owner, spender), &value);
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<(), &'static str> {
            let spender = self.env().caller();
            let allowance = self.allowances.get((from, spender)).unwrap_or(0);
            if allowance < value {
                return Err("Allowance exceeded");
            }
            self.allowances.insert((from, spender), &(allowance - value));
            self._transfer(from, to, value)
        }

        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, value: Balance) -> Result<(), &'static str> {
            let caller = self.env().caller();
            self._mint(caller, to, value)
        }

        #[ink(message)]
        pub fn burn(&mut self, value: Balance) -> Result<(), &'static str> {
            let caller = self.env().caller();
            self._burn(caller, value)
        }

        fn _transfer(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<(), &'static str> {
            if self.balance_of(from) < value {
                return Err("Insufficient balance");
            }
            self.balances.insert(from, &(self.balance_of(from) - value));
            self.balances.insert(to, &(self.balance_of(to) + value));
            self.env().emit_event(Transfer { from, to, value });
            Ok(())
        }

        fn _mint(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<(), &'static str> {
            self.total_supply += value;
            self.balances.insert(to, &(self.balance_of(to) + value));
            self.env().emit_event(Transfer { from: AccountId::from([0; 32]), to, value });
            Ok(())
        }

        fn _burn(&mut self, from: AccountId, value: Balance) -> Result<(), &'static str> {
            if self.balance_of(from) < value {
                return Err("Insufficient balance");
            }
            self.total_supply -= value;
            self.balances.insert(from, &(self.balance_of(from) - value));
            self.env().emit_event(Transfer { from, to: AccountId::from([0; 32]), value });
            Ok(())
        }
    }
}
