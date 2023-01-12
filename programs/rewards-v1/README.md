# Rewards Program

The rewards program is responsible for

- Store data on where to search for bounties
- Storing link between social media profile (or dao) and wallet
- Create NFT that represents reward earning profile
- Transferring bounty to escrow
- Paying out bounty

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
