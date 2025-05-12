const { ethers } = require("hardhat");

async function main() {
    // Replace with your deployed contract address
    const tokenAddress = "YOUR_DEPLOYED_TOKEN_ADDRESS";

    // Get the contract instance
    const Token = await ethers.getContractFactory("AdvancedERC20Token");
    const token = Token.attach(tokenAddress);

    // Get the signer (the account that will interact with the contract)
    const [signer] = await ethers.getSigners();

    // Check the balance of the signer
    const balance = await token.balanceOf(signer.address);
    console.log(`Balance of ${signer.address}: ${ethers.utils.formatUnits(balance, 18)} ATK`);

    // Transfer tokens to another address
    const recipient = "RECIPIENT_ADDRESS"; // Replace with the recipient's address
    const amountToSend = ethers.utils.parseUnits("100", 18); // Amount to send (100 ATK)

    console.log(`Transferring ${ethers.utils.formatUnits(amountToSend, 18)} ATK to ${recipient}...`);
    const transferTx = await token.transfer(recipient, amountToSend);
    await transferTx.wait();
    console.log(`Transfer successful! Transaction hash: ${transferTx.hash}`);

    // Check the balance of the recipient
    const recipientBalance = await token.balanceOf(recipient);
    console.log(`Balance of ${recipient}: ${ethers.utils.formatUnits(recipientBalance, 18)} ATK`);

    // Approve an allowance for another address
    const spender = "SPENDER_ADDRESS"; // Replace with the spender's address
    const amountToApprove = ethers.utils.parseUnits("50", 18); // Amount to approve (50 ATK)

    console.log(`Approving ${ethers.utils.formatUnits(amountToApprove, 18)} ATK for ${spender}...`);
    const approveTx = await token.approve(spender, amountToApprove);
    await approveTx.wait();
    console.log(`Approval successful! Transaction hash: ${approveTx.hash}`);

    // Check the allowance
    const allowance = await token.allowance(signer.address, spender);
    console.log(`Allowance of ${spender} to spend ${signer.address}'s tokens: ${ethers.utils.formatUnits(allowance, 18)} ATK`);
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
