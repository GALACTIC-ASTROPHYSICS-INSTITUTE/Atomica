#![cfg_attr(not(feature = "std"), no_std)]

pub use ink::prelude::vec::Vec;
use ink::storage::Mapping;
use ink::contract::ContractEnv;
use ink::env::DefaultEnvironment;
use ink::storage::traits::SpreadAllocate;
use ink::contract::Contract;

#[ink::contract]
mod governance {
    use super::*;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Governance {
        proposals: Mapping<u64, Proposal>,
        proposal_count: u64,
        voting_period: u64,
        votes: Mapping<(u64, AccountId), bool>,
        token: AccountId,
    }

    #[ink(event)]
    pub struct ProposalCreated {
        #[ink(topic)]
        proposal_id: u64,
        #[ink(topic)]
        creator: AccountId,
        description: Vec<u8>,
    }

    #[ink(event)]
    pub struct Voted {
        #[ink(topic)]
        proposal_id: u64,
        #[ink(topic)]
        voter: AccountId,
        #[ink(topic)]
        support: bool,
    }

    #[ink(event)]
    pub struct ProposalExecuted {
        #[ink(topic)]
        proposal_id: u64,
    }

    #[derive(Default, SpreadAllocate)]
    pub struct Proposal {
        description: Vec<u8>,
        vote_count: u64,
        executed: bool,
        end_time: u64,
    }

    impl Governance {
        #[ink(constructor)]
        pub fn new(token: AccountId, voting_period: u64) -> Self {
            let caller = Self::env().caller();
            let mut instance = Self::allocate();
            instance.token = token;
            instance.voting_period = voting_period;
            instance.proposal_count = 0;
            instance
        }

        #[ink(message)]
        pub fn create_proposal(&mut self, description: Vec<u8>) -> u64 {
            let proposal_id = self.proposal_count;
            let end_time = self.env().block_timestamp() + self.voting_period;

            self.proposals.insert(proposal_id, Proposal {
                description,
                vote_count: 0,
                executed: false,
                end_time,
            });

            self.proposal_count += 1;

            self.env().emit_event(ProposalCreated {
                proposal_id,
                creator: self.env().caller(),
                description: self.proposals.get(proposal_id).unwrap().description.clone(),
            });

            proposal_id
        }

        #[ink(message)]
        pub fn vote(&mut self, proposal_id: u64, support: bool) {
            let caller = self.env().caller();
            let proposal = self.proposals.get(proposal_id).expect("Proposal does not exist");

            assert!(self.env().block_timestamp() < proposal.end_time, "Voting period has ended");
            assert!(!self.votes.get((proposal_id, caller)).unwrap_or(false), "You have already voted");

            self.votes.insert((proposal_id, caller), true);
            if support {
                self.proposals.insert(proposal_id, Proposal {
                    vote_count: proposal.vote_count + 1,
                    ..proposal
                });
            }

            self.env().emit_event(Voted {
                proposal_id,
                voter: caller,
                support,
            });
        }

        #[ink(message)]
        pub fn execute_proposal(&mut self, proposal_id: u64) {
            let caller = self.env().caller();
            let mut proposal = self.proposals.get(proposal_id).expect("Proposal does not exist");

            assert!(self.env().block_timestamp() >= proposal.end_time, "Voting period has not ended");
            assert!(!proposal.executed, "Proposal already executed");

            // Here you can define the logic for what happens when a proposal is executed
            // For example, transferring funds, changing state, etc.

            proposal.executed = true;
            self.proposals.insert(proposal_id, proposal);

            self.env().emit_event(ProposalExecuted {
                proposal_id,
            });
        }

        #[ink(message)]
        pub fn get_proposal(&self, proposal_id: u64) -> (Vec<u8>, u64, bool, u64) {
            let proposal = self.proposals.get(proposal_id).expect("Proposal does not exist");
            (proposal.description.clone(), proposal.vote_count, proposal.executed, proposal.end_time)
        }
    }
}
