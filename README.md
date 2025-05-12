Atomica/
│
├── README.md
├── LICENSE
├── .gitignore
│
├── contracts/
│   ├── Solidity/
│   │   ├── ERC20Token.sol
│   │   ├── ERC721Token.sol
│   │   └── MultiSigWallet.sol
│   │
│   ├── Vyper/
│   │   ├── SimpleStorage.vy
│   │   └── Voting.vy
│   │
│   └── Rust/
│       ├── Token.rs
│       └── Governance.rs
│
├── tests/
│   ├── Solidity/
│   │   ├── test_ERC20.js
│   │   └── test_ERC721.js
│   │
│   ├── Vyper/
│   │   ├── test_SimpleStorage.py
│   │   └── test_Voting.py
│   │
│   └── Rust/
│       ├── test_token.rs
│       └── test_governance.rs
│
├── scripts/
│   ├── deploy/
│   │   ├── deploy_ERC20.js
│   │   └── deploy_Voting.py
│   │
│   ├── interact/
│   │   ├── interact_ERC20.js
│   │   └── interact_Voting.py
│   │
│   └── utils/
│       ├── helpers.js
│       └── constants.py
│
├── docs/
│   ├── architecture/
│   │   └── architecture_diagram.png
│   │
│   ├── tutorials/
│   │   ├── Getting_Started.md
│   │   └── Advanced_Usage.md
│   │
│   └── API/
│       ├── Solidity_API.md
│       ├── Vyper_API.md
│       └── Rust_API.md
│
└── examples/
    ├── Solidity/
    │   ├── example_ERC20.js
    │   └── example_ERC721.js
    │
    ├── Vyper/
    │   ├── example_SimpleStorage.py
    │   └── example_Voting.py
    │
    └── Rust/
        ├── example_token.rs
        └── example_governance.rs
