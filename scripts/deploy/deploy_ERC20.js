const { ethers } = require("hardhat");

async function main() {
    // Get the contract factory
    const Token = await ethers.getContractFactory("AdvancedERC20Token");

    // Define the parameters for the token
    const name = "Advanced Token";
    const symbol = "ATK";
    const initialSupply = ethers.utils.parseUnits("1000000", 18); // 1 million tokens

    // Deploy the contract
    const token = await Token.deploy(name, symbol, initialSupply);
    await token.deployed();

    console.log(`Token deployed to: ${token.address}`);

    // Optionally, you can verify the contract on Etherscan or similar services
    // await verifyContract(token.address, [name, symbol, initialSupply]);
}

// Uncomment this function if you want to verify the contract on Etherscan
/*
async function verifyContract(contractAddress, args) {
    console.log("Verifying contract...");
    await run("verify:verify", {
        address: contractAddress,
        constructorArguments: args,
    });
}
*/

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
