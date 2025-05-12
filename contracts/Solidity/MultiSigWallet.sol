// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/access/Ownable.sol";

contract MultiSigWallet is Ownable {
    // Structure to hold transaction details
    struct Transaction {
        address to;
        uint256 value;
        bool executed;
        uint256 confirmations;
        mapping(address => bool) isConfirmed;
    }

    // State variables
    address[] public owners;
    mapping(address => bool) public isOwner;
    Transaction[] public transactions;
    uint256 public requiredConfirmations;

    // Events
    event Deposit(address indexed sender, uint256 amount);
    event TransactionCreated(uint256 indexed transactionId, address indexed to, uint256 value);
    event TransactionConfirmed(uint256 indexed transactionId, address indexed owner);
    event TransactionExecuted(uint256 indexed transactionId);
    event TransactionRevoked(uint256 indexed transactionId, address indexed owner);

    // Modifiers
    modifier onlyOwner() {
        require(isOwner[msg.sender], "Not an owner");
        _;
    }

    modifier transactionExists(uint256 transactionId) {
        require(transactionId < transactions.length, "Transaction does not exist");
        _;
    }

    modifier notExecuted(uint256 transactionId) {
        require(!transactions[transactionId].executed, "Transaction already executed");
        _;
    }

    modifier notConfirmed(uint256 transactionId) {
        require(!transactions[transactionId].isConfirmed[msg.sender], "Transaction already confirmed");
        _;
    }

    // Constructor
    constructor(address[] memory _owners, uint256 _requiredConfirmations) {
        require(_owners.length > 0, "Owners required");
        require(_requiredConfirmations > 0 && _requiredConfirmations <= _owners.length, "Invalid number of required confirmations");

        for (uint256 i = 0; i < _owners.length; i++) {
            address owner = _owners[i];
            require(owner != address(0), "Invalid owner");
            require(!isOwner[owner], "Owner not unique");

            isOwner[owner] = true;
            owners.push(owner);
        }
        requiredConfirmations = _requiredConfirmations;
    }

    // Fallback function to accept Ether
    receive() external payable {
        emit Deposit(msg.sender, msg.value);
    }

    // Create a new transaction
    function createTransaction(address to, uint256 value) public onlyOwner {
        uint256 transactionId = transactions.length;
        transactions.push();
        Transaction storage newTransaction = transactions[transactionId];
        newTransaction.to = to;
        newTransaction.value = value;
        newTransaction.executed = false;
        newTransaction.confirmations = 0;

        emit TransactionCreated(transactionId, to, value);
    }

    // Confirm a transaction
    function confirmTransaction(uint256 transactionId) public onlyOwner transactionExists(transactionId) notExecuted(transactionId) notConfirmed(transactionId) {
        Transaction storage transaction = transactions[transactionId];
        transaction.isConfirmed[msg.sender] = true;
        transaction.confirmations += 1;

        emit TransactionConfirmed(transactionId, msg.sender);

        if (transaction.confirmations >= requiredConfirmations) {
            executeTransaction(transactionId);
        }
    }

    // Execute a confirmed transaction
    function executeTransaction(uint256 transactionId) internal transactionExists(transactionId) notExecuted(transactionId) {
        Transaction storage transaction = transactions[transactionId];
        require(transaction.confirmations >= requiredConfirmations, "Not enough confirmations");

        transaction.executed = true;
        (bool success, ) = transaction.to.call{value: transaction.value}("");
        require(success, "Transaction failed");

        emit TransactionExecuted(transactionId);
    }

    // Revoke a confirmation
    function revokeConfirmation(uint256 transactionId) public onlyOwner transactionExists(transactionId) notExecuted(transactionId) {
        Transaction storage transaction = transactions[transactionId];
        require(transaction.isConfirmed[msg.sender], "Transaction not confirmed");

        transaction.isConfirmed[msg.sender] = false;
        transaction.confirmations -= 1;

        emit TransactionRevoked(transactionId, msg.sender);
    }

    // Get transaction details
    function getTransaction(uint256 transactionId) public view returns (address to, uint256 value, bool executed, uint256 confirmations) {
        Transaction storage transaction = transactions[transactionId];
        return (transaction.to, transaction.value, transaction.executed, transaction.confirmations);
    }

    // Get the number of owners
    function getOwnersCount() public view returns (uint256) {
        return owners.length;
    }

    // Get the list of owners
    function getOwners() public view returns (address[] memory) {
        return owners;
    }
}
