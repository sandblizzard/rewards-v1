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

# TODO

[ ] Create dummy issues
[ ] Separate Bounty, User etc into module
[ ] Use structs from Bounty program
[ ] Implement checking PRs
