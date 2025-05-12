# Solidity API Documentation
=====================================

This document outlines the Solidity API for the Atomica project, detailing the available functions, events, and structures used in the smart contracts.

## Table of Contents
-------------------
1. [Overview](#overview)
2. [Contract Interfaces](#contract-interfaces)
3. [Function Descriptions](#function-descriptions)
4. [Events](#events)
5. [Data Structures](#data-structures)

## Overview
----------------

The Atomica Solidity API provides a set of smart contracts that facilitate various functionalities such as token management, voting mechanisms, and multi-signature wallets. This API is designed to be modular and extensible, allowing developers to build upon the existing contracts.

## Contract Interfaces
-----------------------

### AtomicaToken

The `AtomicaToken` contract implements the ERC20 standard and includes additional features specific to the Atomica ecosystem.

### Voting

The `Voting` contract allows users to create proposals and cast votes.

## Function Descriptions
-------------------------

### AtomicaToken Functions

- **`function transfer(address recipient, uint256 amount) public returns (bool)`**
  - Transfers `amount` tokens to `recipient`.

- **`function approve(address spender, uint256 amount) public returns (bool)`**
  - Approves `spender` to spend `amount` tokens on behalf of the caller.

### Voting Functions

- **`function createProposal(string memory description) public returns (uint256)`**
  - Creates a new proposal with the given `description`.

- **`function vote(uint256 proposalId, bool support) public`**
  - Casts a vote for the proposal identified by `proposalId`.

## Events
----------------

### AtomicaToken Events

- **`event Transfer(address indexed from, address indexed to, uint256 value)`**
  - Emitted when tokens are transferred.

- **`event Approval(address indexed owner, address indexed spender, uint256 value)`**
  - Emitted when approval is granted.

### Voting Events

- **`event ProposalCreated(uint256 indexed proposalId, string description)`**
  - Emitted when a new proposal is created.

- **`event Voted(uint256 indexed proposalId, address indexed voter, bool support)`**
  - Emitted when a vote is cast.

## Data Structures
-------------------

### Proposal

```solidity
struct Proposal {
    string description;
    uint256 votesFor;
    uint256 votesAgainst;
    bool executed;
}
```

### User

```solidity
struct User {
    address wallet;
    uint256 votingPower;
}
```

## Conclusion
----------------

This API documentation serves as a reference for developers working with the Atomica project. For further details, please refer to the source code and additional resources available in the project repository.
