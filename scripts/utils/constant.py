# Network configurations
NETWORKS = {
    "development": {
        "rpc_url": "http://127.0.0.1:8545",
        "chain_id": 1337,
    },
    "rinkeby": {
        "rpc_url": "https://rinkeby.infura.io/v3/YOUR_INFURA_PROJECT_ID",
        "chain_id": 4,
    },
    "mainnet": {
        "rpc_url": "https://mainnet.infura.io/v3/YOUR_INFURA_PROJECT_ID",
        "chain_id": 1,
    },
}

# Token details
TOKEN_NAME = "Advanced Token"
TOKEN_SYMBOL = "ATK"
INITIAL_SUPPLY = 1_000_000 * (10 ** 18)  # 1 million tokens with 18 decimals

# Voting contract parameters
VOTING_PERIOD = 100  # Voting period in seconds

# Other constants
ZERO_ADDRESS = "0x0000000000000000000000000000000000000000"
