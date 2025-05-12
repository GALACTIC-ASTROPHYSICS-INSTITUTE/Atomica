import pytest
from brownie import SimpleStorage, accounts

def test_initial_value():
    account = accounts[0]
    storage = SimpleStorage.deploy({'from': account})
    assert storage.retrieve() == 0

def test_store_value():
    account = accounts[0]
    storage = SimpleStorage.deploy({'from': account})
    storage.store(42, {'from': account})
    assert storage.retrieve() == 42

def test_overwrite_value():
    account = accounts[0]
    storage = SimpleStorage.deploy({'from': account})
    storage.store(42, {'from': account})
    storage.store(100, {'from': account})
    assert storage.retrieve() == 100

def test_multiple_accounts():
    account1 = accounts[0]
    account2 = accounts[1]
    storage = SimpleStorage.deploy({'from': account1})
    storage.store(42, {'from': account1})
    storage.store(100, {'from': account2})
    assert storage.retrieve() == 42  # account1's value should remain

def test_event_emission():
    account = accounts[0]
    storage = SimpleStorage.deploy({'from': account})
    tx = storage.store(42, {'from': account})
    assert 'ValueChanged' in tx.events
    assert tx.events['ValueChanged']['newValue'] == 42
