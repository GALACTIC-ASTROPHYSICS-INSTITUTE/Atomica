import pytest
from brownie import Voting, accounts

def test_initial_candidates():
    account = accounts[0]
    voting = Voting.deploy(["Alice", "Bob"], {'from': account})
    assert voting.getCandidates() == ["Alice", "Bob"]

def test_cast_vote():
    account = accounts[0]
    voting = Voting.deploy(["Alice", "Bob"], {'from': account})
    voting.vote(0, {'from': account})  # Vote for Alice
    assert voting.getVotes(0) == 1

def test_multiple_votes():
    account1 = accounts[0]
    account2 = accounts[1]
    voting = Voting.deploy(["Alice", "Bob"], {'from': account1})
    voting.vote(0, {'from': account1})  # Vote for Alice
    voting.vote(1, {'from': account2})  # Vote for Bob
    assert voting.getVotes(0) == 1
    assert voting.getVotes(1) == 1

def test_vote_restriction():
    account = accounts[0]
    voting = Voting.deploy(["Alice", "Bob"], {'from': account})
    voting.vote(0, {'from': account})
    with pytest.raises(Exception):
        voting.vote(0, {'from': account})  # Should not allow double voting

def test_winner_declaration():
    account1 = accounts[0]
    account2 = accounts[1]
    voting = Voting.deploy(["Alice", "Bob"], {'from': account1})
    voting.vote(0, {'from': account1})  # Vote for Alice
    voting.vote(1, {'from': account2})  # Vote for Bob
    assert voting.declareWinner() in ["Alice", "Bob"]
