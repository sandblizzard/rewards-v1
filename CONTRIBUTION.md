# Contributing to Sandblizzard

Thanks for taking the time to contribute to Sandblizzard. This repository contains the Solana program, TypeScript tooling, and supporting scripts for the rewards workflow, so small, focused changes are easiest to review.

## Getting Started

1. Fork the repository and create a branch from `main`.
2. Install the project dependencies with the package manager configured for the repo.
3. Review the existing README and package scripts before changing program, SDK, or relayer code.
4. Keep changes scoped to one issue or improvement per pull request.

## Development Workflow

Use clear branch names such as `fix/short-description` or `docs/short-description`. Before opening a pull request, check the files you changed and avoid committing generated output, local keys, wallets, or environment-specific files.

Useful commands from the repository root:

```bash
yarn lint
anchor test --skip-deploy
anchor localnet
```

Run the commands that match the area you changed. Documentation-only pull requests do not need a full test run, but code changes should include the relevant lint or test result in the pull request description.

## Pull Requests

A good pull request should include:

- A short summary of what changed.
- The issue or bounty it addresses, when applicable.
- Any commands run locally and their results.
- Notes about follow-up work or known limitations.

Keep pull requests small enough to review in one pass. If a change touches the on-chain program, SDK, and app flow together, explain why those changes belong in one PR.

## Code Style

Follow the style already used in the repository. Prefer clear names, small functions, and minimal dependencies. Do not introduce formatting-only churn unless the pull request is specifically about formatting.

## Security

Never commit private keys, wallet seed phrases, local validator keys, API tokens, or production configuration. If you find a security issue, avoid posting sensitive exploit details publicly and contact the maintainers with enough information to reproduce the problem safely.
