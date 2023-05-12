# Sandblizzard Monorepo

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

## TODO

### Prio

- [ ] Move "create bounty" dapp to new dapp

### Backlog

- [x] Create "Connect web2 <-> web3 dapp" - sveltekit
- [ ] Write unit tests for relayer
- [ ] Get bounties from contract rather than based on installations
- [ ] Calculate mint price based on github profile.
- [ ] Go through bounty contract and deploy to mainnet
- [ ] Update status text on github
- [ ] Bounties for PRs
