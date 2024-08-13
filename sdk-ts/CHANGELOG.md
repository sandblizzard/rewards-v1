# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

### [0.0.30](https://github.com/sandblizzard/rewards-v1/compare/v0.0.29...v0.0.30) (2024-08-13)

### [0.0.29](https://github.com/sandblizzard/rewards-v1/compare/v0.0.28...v0.0.29) (2024-08-13)


### Features

* **program:** update program and test. Still fails ([2122789](https://github.com/sandblizzard/rewards-v1/commit/2122789bc7cc2b6cccc4399ad787f719cf8cf964))


### Bug Fixes

* update donated amount rather than concatenate the lsit ([d82546c](https://github.com/sandblizzard/rewards-v1/commit/d82546c20ad5bde72c3dc2050ae2154a81657c78))

### [0.0.28](https://github.com/sandblizzard/rewards-v1/compare/v0.0.27...v0.0.28) (2024-07-10)


### Features

* **sdk:** add getBountyById ([75104b9](https://github.com/sandblizzard/rewards-v1/commit/75104b96974be201912fcc2ea95cb2fc0bf06ec5))

### [0.0.27](https://github.com/sandblizzard/rewards-v1/compare/v0.0.26...v0.0.27) (2024-07-05)

### [0.0.26](https://github.com/sandblizzard/rewards-v1/compare/v0.0.25...v0.0.26) (2024-07-04)


### Bug Fixes

* **sdk:** add bountyId as BN ([340ae3e](https://github.com/sandblizzard/rewards-v1/commit/340ae3e8efb243dbe0676844d9080ea92561d163))

### [0.0.25](https://github.com/sandblizzard/rewards-v1/compare/v0.0.24...v0.0.25) (2024-07-04)

### [0.0.24](https://github.com/sandblizzard/rewards-v1/compare/v0.0.23...v0.0.24) (2024-07-04)


### Bug Fixes

* **sdk:** update bounty program Id ([3035b60](https://github.com/sandblizzard/rewards-v1/commit/3035b607a0216dfd66005b7a7232abc070a84fdf))

### [0.0.23](https://github.com/sandblizzard/rewards-v1/compare/v0.0.22...v0.0.23) (2024-07-04)


### Features

* **bounty:** update program to be able to create bounties and submit solutions ([05d4247](https://github.com/sandblizzard/rewards-v1/commit/05d4247451b18b2754368c87b3690ab980b44b1f))

### [0.0.22](https://github.com/sandblizzard/rewards-v1/compare/v0.0.21...v0.0.22) (2024-07-04)

### [0.0.21](https://github.com/sandblizzard/rewards-v1/compare/v0.0.20...v0.0.21) (2024-06-28)


### Features

* **sdk:** add get domain method ([92686a6](https://github.com/sandblizzard/rewards-v1/commit/92686a6ff2d61174fc1bd27038a093ab43cc36c0))

### [0.0.20](https://github.com/sandblizzard/rewards-v1/compare/v0.0.19...v0.0.20) (2024-06-25)

### [0.0.19](https://github.com/sandblizzard/rewards-v1/compare/v0.0.17...v0.0.19) (2024-06-21)


### Bug Fixes

* update program, change address ([7ad149f](https://github.com/sandblizzard/rewards-v1/commit/7ad149f8a2accf9dc74804d360d0db3bc49cfa18))

### [0.0.17](https://github.com/sandblizzard/rewards-v1/compare/v0.0.16...v0.0.17) (2024-06-03)


### Features

* **program:** update contract address, remove freeze authority in order to list ([7eb2990](https://github.com/sandblizzard/rewards-v1/commit/7eb2990f2397316352d95939fe304198544774cb))
* **SDK:** create ata if not exist ([c6b8cc4](https://github.com/sandblizzard/rewards-v1/commit/c6b8cc412ef3747c074013a9a3474168230969aa))

### [0.0.16](https://github.com/sandblizzard/rewards-v1/compare/v0.0.15...v0.0.16) (2024-05-28)

### [0.0.15](https://github.com/sandblizzard/rewards-v1/compare/v0.0.14...v0.0.15) (2024-05-27)

### [0.0.14](https://github.com/sandblizzard/rewards-v1/compare/v0.0.13...v0.0.14) (2024-05-06)

### [0.0.13](https://github.com/sandblizzard/rewards-v1/compare/v0.0.12...v0.0.13) (2024-04-20)

### 0.0.12 (2024-04-20)


### Features

* add bounty denomination ([#35](https://github.com/sandblizzard/rewards-v1/issues/35)) ([67436fa](https://github.com/sandblizzard/rewards-v1/commit/67436fa78f9ff61a922110f7fbe98e84cc594732))
* Add dapp for connecting web2 identities with wallets ([#31](https://github.com/sandblizzard/rewards-v1/issues/31)) ([91c8815](https://github.com/sandblizzard/rewards-v1/commit/91c881596eabc4e66b27a727b5e87897b6876d4b))
* create instructions for creating and deactivating domains ([a347219](https://github.com/sandblizzard/rewards-v1/commit/a3472190f95bf82473a9be9d14045aa029f864ea))
* rewrite the relayer to work with threads, update the deployment script, add remove_relayer instruction ([#30](https://github.com/sandblizzard/rewards-v1/issues/30)) ([1607abf](https://github.com/sandblizzard/rewards-v1/commit/1607abf753d5830c2b5bb99681b179c4d5282350))
* Small PoC on using underdog and relaying bounty creation ([#2](https://github.com/sandblizzard/rewards-v1/issues/2)) ([c2dcb57](https://github.com/sandblizzard/rewards-v1/commit/c2dcb571ec5b59913f7ebc4e1780384acf016cb1))


### Bug Fixes

* bug fix Box<[u8]> error in bounty_state account. Resulted in failure in building IDL ([d19a1a6](https://github.com/sandblizzard/rewards-v1/commit/d19a1a60055d62158a67d942e75f57b180d4bd2f))
* capturing of amount ([c54e68e](https://github.com/sandblizzard/rewards-v1/commit/c54e68e26903c931a1a1ed71408d0dee1ea9a4a2))
* **deployment:** authenticate against aws ecr ([49e71d9](https://github.com/sandblizzard/rewards-v1/commit/49e71d9e6aaa23d0eadaaca429b8c7519f9ea29f))
* enable bounty_proto tests ([37a3e97](https://github.com/sandblizzard/rewards-v1/commit/37a3e97d6169d4178d1663816c3206b24079a361))
* flatten search domains ([ba04590](https://github.com/sandblizzard/rewards-v1/commit/ba0459074b8c5d33c48223d70521b0d2a2109817))
* generate domain_data type by anchorDeserialize instead of borshDeserialize ([f22f332](https://github.com/sandblizzard/rewards-v1/commit/f22f332a77d59bb70f39d4d196d3dbdb98186e80))
* include .yarn ([6e65377](https://github.com/sandblizzard/rewards-v1/commit/6e653771b1a4d51f3c8d5406dd7c33da1ff1cd17))
* move app content to dapp. ([#38](https://github.com/sandblizzard/rewards-v1/issues/38)) ([9eb5204](https://github.com/sandblizzard/rewards-v1/commit/9eb5204d7fa7c04001ca1fd8ce6a05eefe75bab8))
* passing all tests ([61f8166](https://github.com/sandblizzard/rewards-v1/commit/61f8166a18a73d145bda514ec008d81d705422bf))
* **program:** typo ([c6b38f6](https://github.com/sandblizzard/rewards-v1/commit/c6b38f60bb7d97a217c4dd1c8747c1985125cf7d))
* **relayer:** update relayer dockerfile ([#41](https://github.com/sandblizzard/rewards-v1/issues/41)) ([bc2fb54](https://github.com/sandblizzard/rewards-v1/commit/bc2fb542900be2a40716060c38c7c69e8cb67e81))
* update cli, generate new program address ([b433a52](https://github.com/sandblizzard/rewards-v1/commit/b433a5228e1ac69d2981f9385c84c1db94cef1a3))
* update issue handling ([b835595](https://github.com/sandblizzard/rewards-v1/commit/b835595144dd6206591a2606684188393fd094d0))
* use devnet bonk tokens ([35ced36](https://github.com/sandblizzard/rewards-v1/commit/35ced36121ffa70e745f0dbb06f50fff16b90032))

### [0.0.11](https://github.com/sandblizzard/rewards-v1/compare/v0.0.10...v0.0.11) (2023-11-12)


### Bug Fixes

* versions to build program correctly. Disabled relayer and cli atm ([d41be1a](https://github.com/sandblizzard/rewards-v1/commit/d41be1a7a13b0d13f24880cad4a97f0fb99096a4))

### [0.0.10](https://github.com/sandblizzard/rewards-v1/compare/v0.0.9...v0.0.10) (2023-11-08)

### [0.0.9](https://github.com/sandblizzard/rewards-v1/compare/v0.0.8...v0.0.9) (2023-10-30)

### [0.0.8](https://github.com/sandblizzard/rewards-v1/compare/v0.0.7...v0.0.8) (2023-10-30)

### [0.0.7](https://github.com/sandblizzard/rewards-v1/compare/v0.0.6...v0.0.7) (2023-10-30)


### Features

* update program ([d483f38](https://github.com/sandblizzard/rewards-v1/commit/d483f389c7911473de38a3a673bb4fcf32a1a474))

### [0.0.6](https://github.com/sandblizzard/rewards-v1/compare/v0.0.5...v0.0.6) (2023-10-24)

### [0.0.5](https://github.com/sandblizzard/rewards-v1/compare/v0.0.4...v0.0.5) (2023-10-24)

### [0.0.4](https://github.com/sandblizzard/rewards-v1/compare/v0.0.3...v0.0.4) (2023-10-24)

### [0.0.3](https://github.com/sandblizzard/rewards-v1/compare/v0.0.2...v0.0.3) (2023-10-24)

### 0.0.2 (2023-10-24)


### Features

* add bounty denomination ([#35](https://github.com/sandblizzard/rewards-v1/issues/35)) ([67436fa](https://github.com/sandblizzard/rewards-v1/commit/67436fa78f9ff61a922110f7fbe98e84cc594732))
* Add dapp for connecting web2 identities with wallets ([#31](https://github.com/sandblizzard/rewards-v1/issues/31)) ([91c8815](https://github.com/sandblizzard/rewards-v1/commit/91c881596eabc4e66b27a727b5e87897b6876d4b))
* create instructions for creating and deactivating domains ([a347219](https://github.com/sandblizzard/rewards-v1/commit/a3472190f95bf82473a9be9d14045aa029f864ea))
* remove nft reference and simplify program ([44bef4e](https://github.com/sandblizzard/rewards-v1/commit/44bef4ee62b01900863026a79c5507ad46afee6c))
* rewrite the relayer to work with threads, update the deployment script, add remove_relayer instruction ([#30](https://github.com/sandblizzard/rewards-v1/issues/30)) ([1607abf](https://github.com/sandblizzard/rewards-v1/commit/1607abf753d5830c2b5bb99681b179c4d5282350))
* **sdk-ts:** write out bounty sdk to be used by clients ([ab7b887](https://github.com/sandblizzard/rewards-v1/commit/ab7b887dfa1db0a104cf71900ed42c5057967ea9))
* Small PoC on using underdog and relaying bounty creation ([#2](https://github.com/sandblizzard/rewards-v1/issues/2)) ([c2dcb57](https://github.com/sandblizzard/rewards-v1/commit/c2dcb571ec5b59913f7ebc4e1780384acf016cb1))


### Bug Fixes

* bug fix Box<[u8]> error in bounty_state account. Resulted in failure in building IDL ([d19a1a6](https://github.com/sandblizzard/rewards-v1/commit/d19a1a60055d62158a67d942e75f57b180d4bd2f))
* capturing of amount ([c54e68e](https://github.com/sandblizzard/rewards-v1/commit/c54e68e26903c931a1a1ed71408d0dee1ea9a4a2))
* **deployment:** authenticate against aws ecr ([49e71d9](https://github.com/sandblizzard/rewards-v1/commit/49e71d9e6aaa23d0eadaaca429b8c7519f9ea29f))
* enable bounty_proto tests ([37a3e97](https://github.com/sandblizzard/rewards-v1/commit/37a3e97d6169d4178d1663816c3206b24079a361))
* flatten search domains ([ba04590](https://github.com/sandblizzard/rewards-v1/commit/ba0459074b8c5d33c48223d70521b0d2a2109817))
* generate domain_data type by anchorDeserialize instead of borshDeserialize ([f22f332](https://github.com/sandblizzard/rewards-v1/commit/f22f332a77d59bb70f39d4d196d3dbdb98186e80))
* include .yarn ([6e65377](https://github.com/sandblizzard/rewards-v1/commit/6e653771b1a4d51f3c8d5406dd7c33da1ff1cd17))
* move app content to dapp. ([#38](https://github.com/sandblizzard/rewards-v1/issues/38)) ([9eb5204](https://github.com/sandblizzard/rewards-v1/commit/9eb5204d7fa7c04001ca1fd8ce6a05eefe75bab8))
* **relayer:** update relayer dockerfile ([#41](https://github.com/sandblizzard/rewards-v1/issues/41)) ([bc2fb54](https://github.com/sandblizzard/rewards-v1/commit/bc2fb542900be2a40716060c38c7c69e8cb67e81))
* update cli, generate new program address ([b433a52](https://github.com/sandblizzard/rewards-v1/commit/b433a5228e1ac69d2981f9385c84c1db94cef1a3))
* update issue handling ([b835595](https://github.com/sandblizzard/rewards-v1/commit/b835595144dd6206591a2606684188393fd094d0))
* use devnet bonk tokens ([35ced36](https://github.com/sandblizzard/rewards-v1/commit/35ced36121ffa70e745f0dbb06f50fff16b90032))
