# Advanced Usage of Atomica
=====================================

Welcome to the Advanced Usage guide for the Atomica project. This document provides detailed instructions on how to leverage the advanced features of the Atomica ecosystem, including custom contract interactions, multi-signature wallets, and integration with decentralized applications (dApps).

## Table of Contents
-------------------
1. [Custom Contract Interactions](#custom-contract-interactions)
2. [Multi-Signature Wallets](#multi-signature-wallets)
3. [Integrating with dApps](#integrating-with-dapps)
4. [Advanced Voting Mechanisms](#advanced-voting-mechanisms)
5. [Testing and Debugging](#testing-and-debugging)

## Custom Contract Interactions
-------------------------------

### Interacting with Multiple Contracts

Atomica allows you to interact with multiple smart contracts seamlessly. You can create scripts that call functions across different contracts. Here’s an example of how to interact with both the ERC20 token and the voting contract:

```javascript
const { ethers } = require("hardhat");
const { toWei } = require("./utils/helpers");

async function main() {
    const [signer] = await ethers.getSigners();
    const tokenContract = await ethers.getContractAt("AtomicaToken", "YOUR_TOKEN_ADDRESS");
    const votingContract = await ethers.getContractAt("Voting", "YOUR_VOTING_ADDRESS");

    // Transfer tokens to the voting contract
    const amount = toWei(100);
    await tokenContract.transfer(votingContract.address, amount);
    console.log(`Transferred ${amount} tokens to the voting contract.`);
}
```

## Multi-Signature Wallets
---------------------------

### Setting Up a Multi-Signature Wallet

For enhanced security, you can use a multi-signature wallet to manage funds and execute transactions. Here’s how to set up a multi-signature wallet using Gnosis Safe:

1. **Create a Gnosis Safe**: Go to the [Gnosis Safe website](https://gnosis-safe.io/) and create a new safe.
2. **Add Owners**: Add the addresses of the owners who will have access to the wallet.
3. **Set Threshold**: Define the number of signatures required to execute a transaction.

### Interacting with the Multi-Signature Wallet

You can interact with the multi-signature wallet in your scripts:

```javascript
const { ethers } = require("hardhat");

async function main() {
    const safeAddress = "YOUR_GNOSIS_SAFE_ADDRESS";
    const safeContract = await ethers.getContractAt("GnosisSafe", safeAddress);

    // Propose a transaction
    const txData = tokenContract.interface.encodeFunctionData("transfer", [recipientAddress, amount]);
    await safeContract.submitTransaction(tokenContract.address, amount, txData);
    console.log("Transaction proposed to the multi-signature wallet.");
}
```

## Integrating with dApps
---------------------------

### Building a Frontend

You can build a frontend for your dApp using frameworks like React or Vue.js. Use libraries like `ethers.js` or `web3.js` to interact with your smart contracts.

#### Example: Connecting to MetaMask

```javascript
import { ethers } from "ethers";

async function connectWallet() {
    if (window.ethereum) {
        const provider = new ethers.providers.Web3Provider(window.ethereum);
        await provider.send("eth_requestAccounts", []);
        const signer = provider.getSigner();
        console.log("Wallet connected:", await signer.getAddress());
    } else {
        console.error("Please install MetaMask!");
    }
}
```

## Advanced Voting Mechanisms
-------------------------------

### Implementing Delegated Voting

You can implement delegated voting in your voting contract to allow users to delegate their voting power to others. This can be done by adding a `delegate` function in your voting contract.

### Example: Delegating Votes

```solidity
function delegate(address to) public {
    require(to != msg.sender, "Cannot delegate to yourself");
    delegates[msg.sender] = to;
}
```

## Testing and Debugging
-------------------------

### Writing Tests

Use the Hardhat testing framework to write tests for your smart contracts. Here’s an example of a test for the voting contract:

```javascript
const { expect } = require("chai");

describe("Voting Contract", function () {
    it("Should allow users to vote", async function () {
        const Voting = await ethers.getContractFactory("Voting");
        const voting = await Voting.deploy();
        await voting.deployed();

        await voting.createProposal("Proposal 1");
        await voting.vote(0, true);
        
        const proposal = await voting.getProposal(0);
        expect(proposal.votesFor).to.equal(1);
    });
});
```

### Debugging Transactions

Use Hardhat’s built-in debugging tools to trace transactions and identify issues. You can run your tests with the `--show-stack-tr ace` option to get detailed error messages and stack traces.

## Conclusion
----------------

This guide provides an overview of advanced usage scenarios within the Atomica project. By leveraging these features, you can enhance the functionality and security of your smart contracts and dApps. For further assistance, refer to the documentation or reach out to the community. Happy coding!
