const { expect } = require("chai")const { ethers } = require("hardhat");

describe("ERC721 Token Contract", function () {
    let NFT;
    let nft;
    let owner;
    let addr1;
    let addr2;
    let addrs;

    beforeEach(async function () {
        NFT = await ethers.getContractFactory("AdvancedERC721Token");
        [owner, addr1, addr2, ...addrs] = await ethers.getSigners();
        nft = await NFT.deploy("Test NFT", "TNFT");
        await nft.deployed();
    });

    describe("Deployment", function () {
        it("Should set the right owner", async function () {
            expect(await nft.owner()).to.equal(owner.address);
        });

        it("Should have the correct name and symbol", async function () {
            expect(await nft.name()).to.equal("Test NFT");
            expect(await nft.symbol()).to.equal("TNFT");
        });
    });

    describe("Minting", function () {
        it("Should mint a new NFT", async function () {
            await nft.mint(addr1.address, 1);
            expect(await nft.ownerOf(1)).to.equal(addr1.address);
        });

        it("Should fail if minting to the zero address", async function () {
            await expect(nft.mint(ethers.constants.AddressZero, 1)).to.be.revertedWith("ERC721: mint to the zero address");
        });

        it("Should fail if token ID already exists", async function () {
            await nft.mint(addr1.address, 1);
            await expect(nft.mint(addr2.address, 1)).to.be.revertedWith("ERC721: token already minted");
        });
    });

    describe("Transfers", function () {
        beforeEach(async function () {
            await nft.mint(addr1.address, 1);
        });

        it("Should transfer NFT between accounts", async function () {
            await nft.connect(addr1).transferFrom(addr1.address, addr2.address, 1);
            expect(await nft.ownerOf(1)).to.equal(addr2.address);
        });

        it("Should fail if sender is not the owner", async function () {
            await expect(nft.connect(addr2).transferFrom(addr1.address, addr2.address, 1)).to.be.revertedWith("ERC721: transfer caller is not owner nor approved");
        });
    });

    describe("Approval", function () {
        beforeEach(async function () {
            await nft.mint(addr1.address, 1);
        });

        it("Should approve another address to transfer NFT", async function () {
            await nft.connect(addr1).approve(addr2.address, 1);
            expect(await nft.getApproved(1)).to.equal(addr2.address);
        });

        it("Should transfer NFT on behalf of the owner", async function () {
            await nft.connect(addr1).approve(addr2.address, 1);
            await nft.connect(addr2).transferFrom(addr1.address, addr2.address, 1);
            expect(await nft.ownerOf(1)).to.equal(addr2.address);
        });

        it("Should fail if approval is not granted", async function () {
            await expect(nft.connect(addr2).transferFrom(addr1.address, addr2.address, 1)).to.be.revertedWith("ERC721: transfer caller is not owner nor approved");
        });
    });

    describe("Events", function () {
        it("Should emit Transfer event on transfers", async function () {
            await nft.mint(addr1.address, 1);
            await expect(nft.connect(addr1).transferFrom(addr1.address, addr2.address, 1))
                .to.emit(nft, "Transfer")
                .withArgs(addr1.address, addr2.address, 1);
        });

        it("Should emit Approval event on approvals", async function () {
            await nft.mint(addr1.address, 1);
            await expect(nft.connect(addr1).approve(addr2.address, 1))
                .to.emit(nft, "Approval")
                .withArgs(addr1.address, addr2.address, 1);
        });
    });
});
