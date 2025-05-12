const { ethers } = require("hardhat");

/**
 * Converts a value from ether to wei.
 * @param {string|number} value - The value in ether.
 * @returns {BigNumber} - The value in wei.
 */
function toWei(value) {
    return ethers.utils.parseEther(value.toString());
}

/**
 * Converts a value from wei to ether.
 * @param {BigNumber} value - The value in wei.
 * @returns {string} - The value in ether.
 */
function fromWei(value) {
    return ethers.utils.formatEther(value);
}

/**
 * Waits for a transaction to be mined and returns the receipt.
 * @param {Promise<TransactionResponse>} tx - The transaction response.
 * @returns {Promise<TransactionReceipt>} - The transaction receipt.
 */
async function waitForTransaction(tx) {
    const receipt = await tx.wait();
    console.log(`Transaction mined in block: ${receipt.blockNumber}`);
    return receipt;
}

/**
 * Logs the balance of an account.
 * @param {string} address - The address of the account.
 * @param {Contract} tokenContract - The token contract instance.
 */
async function logBalance(address, tokenContract) {
    const balance = await tokenContract.balanceOf(address);
    console.log(`Balance of ${address}: ${ethers.utils.formatUnits(balance, 18)} tokens`);
}

/**
 * Approves a spender to spend tokens on behalf of the owner.
 * @param {Contract} tokenContract - The token contract instance.
 * @param {string} spender - The address of the spender.
 * @param {string|number} amount - The amount of tokens to approve.
 * @param {string} owner - The address of the token owner.
 */
async function approveTokens(tokenContract, spender, amount, owner) {
    const tx = await tokenContract.approve(spender, toWei(amount));
    await waitForTransaction(tx);
    console.log(`Approved ${amount} tokens for ${spender}`);
}

module.exports = {
    toWei,
    fromWei,
    waitForTransaction,
    logBalance,
    approveTokens,
};
