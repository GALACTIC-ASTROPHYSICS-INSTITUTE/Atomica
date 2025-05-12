# SPDX-License-Identifier: MIT
# Voting.vy

# This contract allows users to create proposals and vote on them.
# It includes access control for proposal creation and voting.

# Events
event ProposalCreated:
    proposalId: uint256
    description: string
    creator: address

event Voted:
    proposalId: uint256
    voter: address
    vote: bool

event ProposalExecuted:
    proposalId: uint256

# Proposal structure
struct Proposal:
    description: string
    voteCount: uint256
    executed: bool
    voters: public(map(address, bool))

# State variables
proposals: public(map(uint256, Proposal))
proposalCount: public(uint256)
owner: public(address)

@public
def __init__():
    """
    Initializes the contract and sets the owner to the address that deploys the contract.
    """
    self.owner = msg.sender
    self.proposalCount = 0

@public
def create_proposal(description: string):
    """
    Creates a new proposal. Only the owner can create proposals.
    """
    assert msg.sender == self.owner, "Only the owner can create proposals"
    
    proposalId: uint256 = self.proposalCount
    self.proposals[proposalId] = Proposal({
        description: description,
        voteCount: 0,
        executed: False,
        voters: {}
    })
    
    self.proposalCount += 1
    log ProposalCreated(proposalId, description, msg.sender)

@public
def vote(proposalId: uint256, support: bool):
    """
    Votes on a proposal. Each address can only vote once per proposal.
    """
    assert proposalId < self.proposalCount, "Proposal does not exist"
    proposal: Proposal = self.proposals[proposalId]
    
    assert not proposal.voters[msg.sender], "You have already voted on this proposal"
    
    proposal.voters[msg.sender] = True
    if support:
        proposal.voteCount += 1
    
    log Voted(proposalId, msg.sender, support)

@public
def execute_proposal(proposalId: uint256):
    """
    Executes a proposal if it has enough votes. Only the owner can execute proposals.
    """
    assert msg.sender == self.owner, "Only the owner can execute proposals"
    assert proposalId < self.proposalCount, "Proposal does not exist"
    
    proposal: Proposal = self.proposals[proposalId]
    assert not proposal.executed, "Proposal already executed"
    
    # Here you can define the logic for what happens when a proposal is executed
    # For example, transferring funds, changing state, etc.
    
    proposal.executed = True
    log ProposalExecuted(proposalId)

@public
@constant
def get_proposal(proposalId: uint256) -> (string, uint256, bool):
    """
    Returns the details of a proposal.
    """
    assert proposalId < self.proposalCount, "Proposal does not exist"
    proposal: Proposal = self.proposals[proposalId]
    return (proposal.description, proposal.voteCount, proposal.executed)
