from brownie import Voting, accounts, config

def main():
    # Get the account to deploy the contract
    deployer_account = accounts.add(config["wallets"]["from_key"])

    # Define the candidates for the voting
    candidates = ["Alice", "Bob", "Charlie"]

    # Deploy the Voting contract
    voting_contract = Voting.deploy(candidates, {'from': deployer_account})

    print(f"Voting contract deployed at: {voting_contract.address}")

    # Optionally, you can print the initial state of the contract
    print("Initial candidates:")
    for candidate in candidates:
        print(candidate)

    # Optionally, you can verify the contract on Etherscan or similar services
    # verify_contract(voting_contract.address, candidates)

# Uncomment this function if you want to verify the contract on Etherscan
'''
def verify_contract(contract_address, candidates):
    print("Verifying contract...")
    from brownie import network
    if network.show_active() == "mainnet-fork":
        return
    # Add verification logic here if needed
    # e.g., run("verify:verify", {"address": contract_address, "constructor_arguments": candidates})
'''
