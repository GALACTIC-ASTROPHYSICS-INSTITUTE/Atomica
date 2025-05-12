# SimpleStorage contract in Vyper

storedData: public(uint256)

@public
def set(x: uint256):
    self.storedData = x

@public
@constant
def get() -> uint256:
    return self.storedData
