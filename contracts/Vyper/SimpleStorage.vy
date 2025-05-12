# SPDX-License-Identifier: MIT
# SimpleStorage.vy

# This contract allows users to store and retrieve a single integer value.
# It includes access control for setting the value and emits events for state changes.

# Events
event ValueSet:
    value: uint256
    setter: address

# State variable to store the value
storedValue: public(uint256)
owner: public(address)

@public
@constant
def get_value() -> uint256:
    """
    Returns the stored value.
    """
    return self.storedValue

@public
def set_value(value: uint256):
    """
    Sets the stored value. Only the owner can set the value.
    """
    assert msg.sender == self.owner, "Only the owner can set the value"
    self.storedValue = value
    log ValueSet(value, msg.sender)

@public
def __init__():
    """
    Initializes the contract and sets the owner to the address that deploys the contract.
    """
    self.owner = msg.sender
