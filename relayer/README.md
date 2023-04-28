# Sandblizzard Client

## About

The general idea is that anyone can run the client and relay rewards.

### Main responsibilities of the client are

- Read the repos to be indexed
- Run periodically
- Record the profile <-> wallet pairs in the Solana program
- When a new bounty is created, this should be relayed to the Solana program
- The client should also post the status of the bounty as a way to notify users

### Potential responsibilites

- Trigger transactions in a foreign context like github.com

## Run

```
> RUST_LOG=info cargo run
```

## Prebuild

```
> cargo fmt // formatting
> cargo fix // try to fix warnings
```

# User flows

## Github flow

### Preliminary:

- Install SandBlizzard github app
- Register github username with wallet in repo

### Bounty

1. CREATOR creates an issue or PR
2. Receives a signing url in the comment
3. Signs `create_bounty` tx
4. Closes issue or PR with names of SOLVERS who should receive reward
5. Relayer pays to the SOLVERS from the escrow
6. Relayer posts status to Issue or PR

BOUNTY_CREATOR can always `cancel_bounty` to get the funds out.

# TODO

- [x] Create dummy issues
- [x] Separate Bounty, User etc into module
- [x] Use structs from Bounty program
- [ ] Implement PR flow
- [x] Implement account verification
- [ ] Integrate with bounty program
- [x] Post status about create_bounty to the issue/PR
- [ ] Deploy executable with terraform or pulumi
- [ ] Be able to change payer address
- [ ] Add web server for hosting dapp
- [ ] (Implement JPEG module for creation of new NFT JPEGS)
- [ ] Use bounties created in SC to get indexable domains
