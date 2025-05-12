// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

contract AdvancedERC721Token is ERC721URIStorage, Ownable, Pausable {
    using Counters for Counters.Counter;
    Counters.Counter private _tokenIdCounter;

    // Mapping from token ID to token metadata
    mapping(uint256 => string) private _tokenURIs;

    // Events
    event Minted(uint256 indexed tokenId, address indexed owner, string tokenURI);
    event Burned(uint256 indexed tokenId);
    event BaseURIUpdated(string newBaseURI);

    constructor(string memory name, string memory symbol) ERC721(name, symbol) {}

    // Function to mint a new token
    function mint(string memory tokenURI) public whenNotPaused onlyOwner {
        uint256 tokenId = _tokenIdCounter.current();
        _mint(msg.sender, tokenId);
        _setTokenURI(tokenId, tokenURI);
        _tokenIdCounter.increment();
        emit Minted(tokenId, msg.sender, tokenURI);
    }

    // Function to burn a token
    function burn(uint256 tokenId) public whenNotPaused {
        require(ownerOf(tokenId) == msg.sender, "ERC721: caller is not the owner");
        _burn(tokenId);
        emit Burned(tokenId);
    }

    // Function to set the token URI
    function setTokenURI(uint256 tokenId, string memory tokenURI) public whenNotPaused onlyOwner {
        require(_exists(tokenId), "ERC721: URI set of nonexistent token");
        _setTokenURI(tokenId, tokenURI);
    }

    // Function to pause all token transfers
    function pause() public onlyOwner {
        _pause();
    }

    // Function to unpause all token transfers
    function unpause() public onlyOwner {
        _unpause();
    }

    // Function to update the base URI for all tokens
    function setBaseURI(string memory baseURI) public onlyOwner {
        _setBaseURI(baseURI);
        emit BaseURIUpdated(baseURI);
    }

    // Internal function to set the base URI
    function _setBaseURI(string memory baseURI) internal {
        _baseURI = baseURI;
    }

    // Override the _baseURI function
    string private _baseURI;

    function _baseURI() internal view virtual override returns (string memory) {
        return _baseURI;
    }
}
