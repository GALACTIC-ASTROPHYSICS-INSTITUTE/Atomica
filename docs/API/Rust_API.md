# Rust API Documentation
=====================================

This document provides an overview of the Rust API for the Atomica project, detailing the available functions, structures, and modules used in the implementation.

## Table of Contents
-------------------
1. [Overview](#overview)
2. [Modules](#modules)
3. [Function Descriptions](#function-descriptions)
4. [Data Structures](#data-structures)

## Overview
----------------

The Atomica Rust API is designed to offer a robust and efficient interface for interacting with the Atomica ecosystem. It leverages Rust's performance and safety features to ensure secure and high-performance operations.

## Modules
-----------------------

### atomica_token

The `atomica_token` module provides functionalities related to token management, including creation, transfer, and approval of tokens.

### governance

The `governance` module facilitates proposal creation and voting mechanisms within the Atomica ecosystem.

## Function Descriptions
-------------------------

### atomica_token Functions

- **`fn transfer(recipient: &str, amount: u64) -> Result<(), String>`**
  - Transfers `amount` tokens to the specified `recipient`.

- **`fn approve(spender: &str, amount: u64) -> Result<(), String>`**
  - Approves `spender` to spend `amount` tokens on behalf of the caller.

### governance Functions

- **`fn create_proposal(description: &str) -> Result<u64, String>`**
  - Creates a new proposal with the provided `description`.

- **`fn vote(proposal_id: u64, support: bool) -> Result<(), String>`**
  - Casts a vote for the proposal identified by `proposal_id`.

## Data Structures
-------------------

### Proposal

```rust
struct Proposal {
    description: String,
    votes_for: u64,
    votes_against: u64,
    executed: bool,
}
```

### User

```rust
struct User {
    wallet: String,
    voting_power: u64,
}
```

## Conclusion
----------------

This API documentation serves as a guide for developers working with the Atomica project in Rust. For more information, please refer to the source code and additional resources available in the project repository.
