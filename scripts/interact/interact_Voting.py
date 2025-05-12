from brownie import Voting, accounts, config

def main():
    # Get the account to interact with the contract
    user_account = accounts.add(config["wallets"]["from_key"])

    # Replace with your deployed contract address
    voting_contract_address = "YOUR_DEPLOYED_VOTING_CONTRACT_ADDRESS"
    voting_contract = Voting.at(voting_contract_address)

    # Display the current proposals
    print("Current Proposals:")
    for i in range(voting_contract.proposal_count()):
        proposal = voting_contract.get_proposal(i)
        print(f"ID: {i}, Description: {proposal[0].decode('utf-8')}, Votes: {proposal[1]}, Executed: {proposal[2]}")

    # Create a new proposal
    new_proposal_description = "Increase the budget for community projects".encode('utf-8')
    print(f"\nCreating a new proposal: {new_proposal_description.decode('utf-8')}")
    tx = voting_contract.create_proposal(new_proposal_description, {'from': user_account})
    tx.wait(1)
    print(f"Proposal created! Transaction hash: {tx.txid}")

    # Vote on a proposal
    proposal_id_to_vote = 0  # Change this to the ID of the proposal you want to vote on
    print(f"\nVoting on proposal ID: {proposal_id_to_vote}")
    tx = voting_contract.vote(proposal_id_to_vote, True, {'from': user_account})  # True for in favor
    tx.wait(1)
    print(f"Voted successfully! Transaction hash: {tx.txid}")

    # Check the results of the proposal
    print("\nChecking proposal results:")
    proposal = voting_contract.get_proposal(proposal_id_to_vote)
    print(f"ID: {proposal_id_to_vote}, Description: {proposal[0].decode('utf-8')}, Votes: {proposal[1]}, Executed: {proposal[2]}")

    # Execute the proposal if the voting period has ended
    print(f"\nAttempting to execute proposal ID: {proposal_id_to_vote}")
    try:
        tx = voting_contract.execute_proposal(proposal_id_to_vote, {'from': user_account})
        tx.wait(1)
        print(f"Proposal executed! Transaction hash: {tx.txid}")
    except Exception as e:
        print(f"Failed to execute proposal: {str(e)}")

