# Vyper API Documentation
=====================================

This document outlines the Vyper API for the Atomica project, detailing the available functions, events, and structures used in the smart contracts.

## Table of Contents
-------------------
1. [Overview](#overview)
2. [Contract Interfaces](#contract-interfaces)
3. [Function Descriptions](#function-descriptions)
4. [Events](#events)
5. [Data Structures](#data-structures)

## Overview
----------------

The Atomica Vyper API provides a set of smart contracts that facilitate various functionalities such as token management, governance, and security features. This API is designed to be simple and secure, adhering to the principles of Vyper.

## Contract Interfaces
-----------------------

### AtomicaTokenVyper

The `AtomicaTokenVyper` contract implements the ERC20 standard with additional security features tailored for the Atomica ecosystem.

### Governance

The `Governance` contract allows users to propose changes and vote on governance matters.

## Function Descriptions
-------------------------

### AtomicaTokenVyper Functions

- **`@public
  @payable
  def transfer(recipient: address, amount: uint256) -> bool:`**
  - Transfers `amount` tokens to `recipient`.

- **`@public
  def approve(spender: address, amount: uint256) -> bool:`**
  - Approves `spender` to spend `amount` tokens on behalf of the caller.

### Governance Functions

- **`@public
  def create_proposal(description: string) -> uint256:`**
  - Creates a new proposal with the given `description`.

- **`@public
  def vote(proposal_id: uint256, support: bool):`**
  - Casts a vote for the proposal identified by `proposal_id`.

## Events
----------------

### AtomicaTokenVyper Events

- **`event Transfer(sender: indexed(address), recipient: indexed(address), value: uint256)`**
  - Emitted when tokens are transferred.

- **`event Approval(owner: indexed(address), spender: indexed(address), value: uint256)`**
  - Emitted when approval is granted.

### Governance Events

- **`event ProposalCreated(proposal_id: indexed(uint256), description: string)`**
  - Emitted when a new proposal is created.

- **`event Voted(proposal_id: indexed(uint256), voter: indexed(address), support: bool)`**
  - Emitted when a vote is cast.

## Data Structures
-------------------

### Proposal

```vyper
struct Proposal:
    description: string
    votes_for: uint256
    votes_against: uint256
    executed: bool
```

### User

```vyper
struct User:
    wallet: address
    voting_power: uint256
```

## Conclusion
----------------

This API documentation serves as a reference for developers working with the Atomica project. For further details, please refer to the source code and additional resources available in the project repository.
