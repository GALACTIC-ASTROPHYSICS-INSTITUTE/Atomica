#![cfg(test)]

use ink::prelude::vec::Vec;
use ink::test;
use ink::env::DefaultEnvironment;
use ink::storage::Mapping;
use ink::contract::Contract;

#[ink::test]
fn test_initial_supply() {
    let token = Token::new("Test Token".to_string(), "TTK".to_string(), 18, 1_000_000);
    let owner_balance = token.balance_of(token.env().caller());
    assert_eq!(owner_balance, 1_000_000);
}

#[ink::test]
fn test_transfer() {
    let mut token = Token::new("Test Token".to_string(), "TTK".to_string(), 18, 1_000_000);
    let accounts = ink::env::test::default_accounts::<DefaultEnvironment>();
    
    // Transfer tokens from owner to addr1
    let transfer_amount = 100;
    token.transfer(accounts.alice, transfer_amount).unwrap();
    
    assert_eq!(token.balance_of(accounts.alice), transfer_amount);
    assert_eq!(token.balance_of(token.env().caller()), 1_000_000 - transfer_amount);
}

#[ink::test]
fn test_transfer_insufficient_balance() {
    let mut token = Token::new("Test Token".to_string(), "TTK".to_string(), 18, 1_000_000);
    let accounts = ink::env::test::default_accounts::<DefaultEnvironment>();
    
    // Attempt to transfer more than balance
    let transfer_amount = 1_000_000 + 1;
    assert!(token.transfer(accounts.bob, transfer_amount).is_err());
}

#[ink::test]
fn test_approve_and_transfer_from() {
    let mut token = Token::new("Test Token".to_string(), "TTK".to_string(), 18, 1_000_000);
    let accounts = ink::env::test::default_accounts::<DefaultEnvironment>();
    
    // Approve addr1 to spend 100 tokens
    token.approve(accounts.alice, 100).unwrap();
    
    // Transfer from owner to addr2 using addr1
    token.transfer_from(token.env().caller(), accounts.bob, 100).unwrap();
    
    assert_eq!(token.balance_of(accounts.bob), 100);
    assert_eq!(token.balance_of(token.env().caller()), 1_000_000 - 100);
}

#[ink::test]
fn test_approve_exceeding_allowance() {
    let mut token = Token::new("Test Token".to_string(), "TTK".to_string(), 18, 1_000_000);
    let accounts = ink::env::test::default_accounts::<DefaultEnvironment>();
    
    // Approve addr1 to spend 100 tokens
    token.approve(accounts.alice, 100).unwrap();
    
    // Attempt to transfer more than allowance
    assert!(token.transfer_from(token.env().caller(), accounts.bob, 101).is_err());
}

#[ink::test]
fn test_mint() {
    let mut token = Token::new("Test Token".to_string(), "TTK".to_string(), 18, 1_000_000);
    let accounts = ink::env::test::default_accounts::<DefaultEnvironment>();
    
    // Mint 500 tokens to addr1
    token.mint(accounts.alice, 500).unwrap();
    
    assert_eq!(token.balance_of(accounts.alice), 500);
    assert_eq!(token.total_supply(), 1_000_000 + 500);
}

#[ink::test]
fn test_burn() {
    let mut token = Token::new("Test Token".to_string(), "TTK".to_string(), 18, 1_000_000);
    let accounts = ink::env::test::default_accounts::<DefaultEnvironment>();
    
    // Burn 100 tokens from owner
    token.burn(100).unwrap();
    
    assert_eq!(token.balance_of(token.env().caller()), 1_000_000 - 100);
    assert_eq!(token.total_supply(), 1_000_000 - 100);
}
