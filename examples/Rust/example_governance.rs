// example_governance.rs

use ink_lang as ink;

#[ink::contract]
mod example_governance {
    #[ink(storage)]
    pub struct ExampleGovernance {
        proposals: ink_storage::collections::HashMap<u32, Proposal>,
        proposal_count: u32,
    }

    #[derive(Default)]
    pub struct Proposal {
        description: String,
        vote_count: u32,
    }

    impl ExampleGovernance {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                proposals: ink_storage::collections::HashMap::new(),
                proposal_count: 0,
            }
        }

        #[ink(message)]
        pub fn create_proposal(&mut self, description: String) -> u32 {
            self.proposal_count += 1;
            let proposal = Proposal {
                description,
                vote_count: 0,
            };
            self.proposals.insert(self.proposal_count, proposal);
            self.proposal_count
        }

        #[ink(message)]
        pub fn vote(&mut self, proposal_id: u32) {
            if let Some(proposal) = self.proposals.get_mut(&proposal_id) {
                proposal.vote_count += 1;
            }
        }
    }
}
