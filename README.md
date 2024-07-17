# Sandblizzard 

This is the monorepo for the submission(s) to the [Sandstorm Solana Hackathon](https://www.sandstormhackathon.com/)

## Run integrationt test

```
> anchor test --skip-deploy
```

## Test locally

```
> anchor localnet
```

The repo contains

- The program for storing and managing payment of rewards
- Client for indexing repositories and storing information on-chain - TBD
- Remote transaction trigger - TBD
- User profile with defi tools -TBD

## Board

### Todo

- [x] Move "create bounty" dapp to new dapp
- [ ] Make sure relayer works as expected
- [ ] bounty issue id should be recorded and created

### Backlog

- [x] Create "Connect web2 <-> web3 dapp" - sveltekit
- [ ] Relayer: Write unit tests for relayer
- [ ] Relayer: Get bounties from contract rather than based on installations
- [ ] Dapp: Calculate mint price based on github profile.
- [ ] Program: Go through bounty contract and deploy to mainnet
- [ ] Relayer: Improve status texts in relayer
- [ ] Dapp: Use candy machine to mint NFTs instead as it's cheaper

### NEXT

- [ ] Bounties for PRs
