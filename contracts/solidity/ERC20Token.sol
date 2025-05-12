// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/Pausable.sol";
import "@openzeppelin/contracts/utils/math/SafeMath.sol";

contract AdvancedERC20Token is ERC20, Ownable, Pausable {
    using SafeMath for uint256;

    // Mapping from account to frozen status
    mapping(address => bool) private _frozenAccounts;

    // Events
    event FrozenFunds(address target, bool frozen);
    event Minted(address indexed to, uint256 amount);
    event Burned(address indexed from, uint256 amount);

    constructor(string memory name, string memory symbol) ERC20(name, symbol) {
        // Initial minting to the contract owner
        _mint(msg.sender, 1000000 * 10 ** decimals()); // Mint 1 million tokens
    }

    // Function to mint new tokens
    function mint(address account, uint256 amount) public onlyOwner {
        require(account != address(0), "ERC20: mint to the zero address");
        _mint(account, amount);
        emit Minted(account, amount);
    }

    // Function to burn tokens
    function burn(uint256 amount) public {
        _burn(msg.sender, amount);
        emit Burned(msg.sender, amount);
    }

    // Function to freeze an account
    function freezeAccount(address target, bool freeze) public onlyOwner {
        _frozenAccounts[target] = freeze;
        emit FrozenFunds(target, freeze);
    }

    // Override transfer functions to prevent transfers from frozen accounts
    function _beforeTokenTransfer(address from, address to, uint256 amount) internal override {
        require(!_frozenAccounts[from], "ERC20: transfer from frozen account");
        require(!_frozenAccounts[to], "ERC20: transfer to frozen account");
        super._beforeTokenTransfer(from, to, amount);
    }

    // Function to pause all token transfers
    function pause() public onlyOwner {
        _pause();
    }

    // Function to unpause all token transfers
    function unpause() public onlyOwner {
        _unpause();
    }

    // Function to check if an account is frozen
    function isFrozen(address account) public view returns (bool) {
        return _frozenAccounts[account];
    }
}
