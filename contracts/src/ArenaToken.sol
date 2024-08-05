pragma solidity ^0.8.10;

import "solmate/src/tokens/ERC20.sol";

contract ArenaToken is ERC20 {
    address public admin;

    constructor(string memory name, string memory symbol, uint8 decimals) ERC20(name, symbol, decimals) {
        admin = msg.sender; // Set the contract deployer as the initial admin
    }

    function mint(uint256 amount) public returns (bool) {
        _mint(msg.sender, amount);
        return true;
    }
}
