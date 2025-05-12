#![cfg(test)]

use ink::prelude::vec::Vec;
use ink::test;
use ink::env::DefaultEnvironment;
use ink::storage::Mapping;
use ink::contract::Contract;

#[ink::test]
fn test_initial_proposal_count() {
    let governance = Governance::new(100); // Voting period of 100 seconds
    assert_eq!(governance.proposal_count(), 0);
}

#[ink::test]
fn test_create_proposal() {
    let mut governance = Governance::new(100);
    let proposal_id = governance.create_proposal("Increase token supply".as_bytes().to_vec()).unwrap();
    
    assert_eq!(governance.proposal_count(), 1);
    let proposal = governance.get_proposal(proposal_id).unwrap();
    assert_eq!(proposal.description, "Increase token supply".as_bytes().to_vec());
    assert_eq!(proposal.vote_count, 0);
    assert!(!proposal.executed);
}

#[ink::test]
fn test_vote_on_proposal() {
    let mut governance = Governance::new(100);
    let proposal_id = governance.create_proposal("Increase token supply".as_bytes().to_vec()).unwrap();
    
    let accounts = ink::env::test::default_accounts::<DefaultEnvironment>();
    governance.vote(proposal_id, true, accounts.alice).unwrap();
    
    let proposal = governance.get_proposal(proposal_id).unwrap();
    assert_eq!(proposal.vote_count, 1);
}

#[ink::test]
fn test_vote_restriction() {
    let mut governance = Governance::new(100);
    let proposal_id = governance.create_proposal("Increase token supply".as_bytes().to_vec()).unwrap();
    
    let accounts = ink::env::test::default_accounts::<DefaultEnvironment>();
    governance.vote(proposal_id, true, accounts.alice).unwrap();
    
    // Attempt to vote again
    assert!(governance.vote(proposal_id, true, accounts.alice).is_err());
}

#[ink::test]
fn test_execute_proposal() {
    let mut governance = Governance::new(100);
    let proposal_id = governance.create_proposal("Increase token supply".as_bytes().to_vec()).unwrap();
    
    let accounts = ink::env::test::default_accounts::<DefaultEnvironment>();
    governance.vote(proposal_id, true, accounts.alice).unwrap();
    
    // Simulate time passing
    ink::env::test::set_block_timestamp(ink::env::test::block_timestamp() + 101);
    
    governance.execute_proposal(proposal_id).unwrap();
    
    let proposal = governance.get_proposal(proposal_id).unwrap();
    assert!(proposal.executed);
}

#[ink::test]
fn test_execute_proposal_before_voting_period() {
    let mut governance = Governance::new(100);
    let proposal_id = governance.create_proposal("Increase token supply".as_bytes().to_vec()).unwrap();
    
    let accounts = ink::env::test::default_accounts::<DefaultEnvironment>();
    governance.vote(proposal_id, true, accounts.alice).unwrap();
    
    // Attempt to execute before voting period ends
    assert!(governance.execute_proposal(proposal_id).is_err());
}

#[ink::test]
fn test_get_proposal() {
    let mut governance = Governance::new(100);
    let proposal_id = governance.create_proposal("Increase token supply".as_bytes().to_vec()).unwrap();
    
    let proposal = governance.get_proposal(proposal_id).unwrap();
    assert_eq!(proposal.description, "Increase token supply".as_bytes().to_vec());
    assert_eq!(proposal.vote_count, 0);
    assert!(!proposal.executed);
}
