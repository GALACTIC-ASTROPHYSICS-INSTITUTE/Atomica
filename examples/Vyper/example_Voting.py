# Voting contract in Vyper

candidates: public(map(uint256, string))
votes: public(map(uint256, uint256))
totalVotes: public(uint256)

@public
def addCandidate(candidateId: uint256, name: string):
    self.candidates[candidateId] = name
    self.votes[candidateId] = 0

@public
def vote(candidateId: uint256):
    self.votes[candidateId] += 1
    self.totalVotes += 1

@public
@constant
def getVotes(candidateId: uint256) -> uint256:
    return self.votes[candidateId]
