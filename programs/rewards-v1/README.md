# Rewards Program

The rewards program is responsible for

- Store data on where to search for bounties
- Storing link between social media profile (or dao) and wallet
- Create NFT that represents reward earning profile
- Transferring bounty to escrow
- Paying out bounty

## Actors

- RELAYER
  - responsible for relaying information from repos to the contract
- BOUNTY_CREATOR
  - Creates a bounty by typing a special keyword in repo, issue etc
  - Signs transactions for transferring to and from escrow
- USER
  - Could be a BOUNTY_CREATOR or BOUNTY_SOLVER
  - The proof of user is an NFT that contains reputation as well as earned amount
- PROTOCOL OWNER
  - Has the ability to update protocol configurations
  - Deployment
  - Adding of new relayers

In the beginning the only relayer will be ran by **Sandblizzard**.

## Sandstorm ideas

- Since payment is in Bonk, allow users to swap using orca
- Allow users to fractionalize their NFT

## Dependencies

- Metaplex for creating NFTs

## Tests

Run `test-bpf` integration tests:

```
> cargo test-bpf --test test -- --nocapture
```

Run `anchor` integration tests

```
> anchor test
```

## Build

```
> anchor build
```

## Deploy

```
> anchor deploy
```

# Contract addresses

TBD
