const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("ERC20 Token Contract", function () {
    let Token;
    let token;
    let owner;
    let addr1;
    let addr2;
    let addrs;

    const initialSupply = ethers.utils.parseUnits("1000000", 18); // 1 million tokens

    beforeEach(async function () {
        Token = await ethers.getContractFactory("AdvancedERC20Token");
        [owner, addr1, addr2, ...addrs] = await ethers.getSigners();
        token = await Token.deploy("Test Token", "TTK", initialSupply);
        await token.deployed();
    });

    describe("Deployment", function () {
        it("Should set the right owner", async function () {
            expect(await token.owner()).to.equal(owner.address);
        });

        it("Should assign the total supply of tokens to the owner", async function () {
            const ownerBalance = await token.balanceOf(owner.address);
            expect(ownerBalance).to.equal(initialSupply);
        });

        it("Should have the correct name and symbol", async function () {
            expect(await token.name()).to.equal("Test Token");
            expect(await token.symbol()).to.equal("TTK");
        });
    });

    describe("Transactions", function () {
        it("Should transfer tokens between accounts", async function () {
            await token.transfer(addr1.address, ethers.utils.parseUnits("100", 18));
            const addr1Balance = await token.balanceOf(addr1.address);
            expect(addr1Balance).to.equal(ethers.utils.parseUnits("100", 18));

            await token.connect(addr1).transfer(addr2.address, ethers.utils.parseUnits("50", 18));
            const addr2Balance = await token.balanceOf(addr2.address);
            expect(addr2Balance).to.equal(ethers.utils.parseUnits("50", 18));
        });

        it("Should fail if sender doesn’t have enough tokens", async function () {
            const initialOwnerBalance = await token.balanceOf(owner.address);
            await expect(token.connect(addr1).transfer(owner.address, ethers.utils.parseUnits("1", 18)))
                .to.be.revertedWith("ERC20: transfer amount exceeds balance");
            expect(await token.balanceOf(owner.address)).to.equal(initialOwnerBalance);
        });

        it("Should update balances after transfers", async function () {
            const initialOwnerBalance = await token.balanceOf(owner.address);
            await token.transfer(addr1.address, ethers.utils.parseUnits("100", 18));
            await token.transfer(addr2.address, ethers.utils.parseUnits("50", 18));

            const finalOwnerBalance = await token.balanceOf(owner.address);
            expect(finalOwnerBalance).to.equal(initialOwnerBalance.sub(ethers.utils.parseUnits("150", 18)));

            const addr1Balance = await token.balanceOf(addr1.address);
            expect(addr1Balance).to.equal(ethers.utils.parseUnits("100", 18));

            const addr2Balance = await token.balanceOf(addr2.address);
            expect(addr2Balance).to.equal(ethers.utils.parseUnits("50", 18));
        });
    });

    describe("Approval and Allowance", function () {
        it("Should approve tokens for delegated transfer", async function () {
            await token.approve(addr1.address, ethers.utils.parseUnits("100", 18));
            const allowance = await token.allowance(owner.address, addr1.address);
            expect(allowance).to.equal(ethers.utils.parseUnits("100", 18));
        });

        it("Should transfer tokens on behalf of the owner", async function () {
            await token.approve(addr1.address, ethers.utils.parseUnits("100", 18));
            await token.connect(addr1).transferFrom(owner.address, addr2.address, ethers.utils.parseUnits("100", 18));

            const addr2Balance = await token.balanceOf(addr2.address);
            expect(addr2Balance).to.equal(ethers.utils.parseUnits("100", 18));

            const ownerBalance = await token.balanceOf(owner.address);
            expect(ownerBalance).to.equal(initialSupply.sub(ethers.utils.parseUnits("100", 18)));
        });

        it("Should fail if allowance is exceeded", async function () {
            await token.approve(addr1.address, ethers.utils.parseUnits("100", 18));
            await expect(token.connect(addr1).transferFrom(owner.address, addr2.address, ethers.utils.parseUnits("101", 18)))
                .to.be.revertedWith("ERC20: transfer amount exceeds allowance");
        });
    });

    describe("Minting and Burning", function () {
        it("Should mint new tokens", async function 
