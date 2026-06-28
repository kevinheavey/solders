# Changelog

# Unreleased

### Changed

- Update litesvm to 0.13 and bump Solana deps as far as litesvm 0.13 allows: `agave-feature-set`/`agave-precompiles`, `solana-compute-budget`, `solana-hash`, `solana-transaction-context` to 4 and `solana-system-interface` to 3 (crates litesvm still pins to the v3 line, e.g. `solana-message`/`solana-transaction`/`solana-account`, stay there)
- Bump the (litesvm-independent) client crates to 4: `solana-rpc-client-api`, `solana-rpc-client-types`, `solana-account-decoder-client-types`, `solana-transaction-status-client-types`, and `solana-reward-info` to 5 (the version aligned with that set, not the newer 6.x)
- `ComputeBudget` constructor now takes `simd_0268_active` and `simd_0339_active` flags (was `simd_0296_active`) to match `solana-compute-budget` 4

### Added

- `commission_bps` on `Reward` and `RpcInflationReward`, `transaction_index` on `RpcConfirmedTransactionStatusWithSignature`, and `client_id` on `RpcContactInfo`, following the upstream v4 client types
- Support for the `getStakeMinimumDelegation` RPC method (`GetStakeMinimumDelegation` request and `GetStakeMinimumDelegationResp` response)
- `ComputeBudget`: getters/setters for the new `solana-compute-budget` 4 cost fields (`alt_bn128_g2_addition_cost`, `alt_bn128_g2_multiplication_cost`, and the `bls12_381_*` set)
- `transaction_index` on `EncodedConfirmedTransactionWithStatusMeta`, following `solana-transaction-status-client-types` 4
- `solders.system_program.create_account_allow_prefund` (and `decode_create_account_allow_prefund` / `CreateAccountAllowPrefundParams`), wrapping the new `solana-system-interface` instruction
- `LiteSVM.with_feature_set` to apply a `FeatureSet` to the VM (the `FeatureSet` type was exposed but could not previously be applied)
- `LiteSVM.with_mainnet_features` / `LiteSVM.mainnet_feature_set` / `LiteSVM.get_sigverify`
- `TransactionMetadata.fee` and `TransactionMetadata.pretty_logs`
- `FeatureSet.activate` / `FeatureSet.deactivate`
- `Rent.with_lamports_per_byte` and the `DEFAULT_LAMPORTS_PER_BYTE` constant (the deprecated rent accessors are kept)
- `EpochRewards.distribute`
- `StakeHistoryEntry.with_effective` / `with_effective_and_activating` / `with_deactivating`
- `LiteSVM.airdrop_pubkey`, `LiteSVM.with_feature_accounts`, and `LiteSVM.add_program_with_loader`
- A getter for `UiConfirmedBlock.num_reward_partitions` (the constructor already accepted it but it could not be read back)
- `solders.system_program.upgrade_nonce_account` (and `decode_upgrade_nonce_account` / `UpgradeNonceAccountParams`)
- Restored pickle and `copy.deepcopy` support: a `__reduce__` method is now generated for all types using the `common_methods` family of macros (reconstructing via `from_bytes(bytes(self))`).
- `copy.deepcopy` support for RPC response types (via a clone-based `__deepcopy__`; these don't support pickle because their bincode round-trip is broken by `skip_serializing_if`).

### Removed

- `GetStakeActivation` request: the `getStakeActivation` RPC method has been removed from Agave and is no longer served by validators

### Fixed

- `StakeHistoryEntry`: the `activating` and `deactivating` setters wrote to the `effective` field instead of their own
- `Rent` declared `module = "solders.account"` but is exported from `solders.rent`, which broke pickling.
- `EncodedConfirmedTransactionWithStatusMeta` now serializes its bytes via CBOR instead of bincode. Its `bytes()`/`from_bytes` were broken (bincode can't represent the `#[serde(flatten)]` field), which also broke pickle and deepcopy for it.

# [0.27.1] 2025-11-15

### Fixed

- Back to not requiring ID params for GetClusterNodes and others [(#164)](https://github.com/kevinheavey/solders/pull/164)

# [0.27.0] - 2025-10-26

### Changed

- Upgrade solana deps to 3.0 [(#156)](https://github.com/kevinheavey/solders/pull/156)
- Remove old unused program-test code [(#155)](https://github.com/kevinheavey/solders/pull/155)

### Fixed

- Add missing getters to `RpcContactInfo` [(#161)](https://github.com/kevinheavey/solders/pull/161)

# [0.26.0] - 2025-02-18

### Fixed

- Fix `parse_websocket_message` when the message is a `jsonParsed` account notification [(#138)](https://github.com/kevinheavey/solders/pull/138)

### Changed

- Upgrade to Solana 2.2 crates [(#137)](https://github.com/kevinheavey/solders/pull/137)
- Remove `from_json` methods in `solders.rpc.requests` [(#137)](https://github.com/kevinheavey/solders/pull/137)

# [0.25.0] - 2025-01-27

### Changed

- Remove `solders.bankrun` in favour of `solders.litesvm`
- Update to pyo3 0.23
- Remove pickle support
- Rename `to_bytes_array` methods to `to_bytes` to reflect changed type

# [0.24.1] - 2025-01-25

### Fixed

- Explicitly add `litesvm` to `__init__.py` [(#133)](https://github.com/kevinheavey/solders/pull/133)

# [0.24.0] - 2025-01-24

### Added

- Add LiteSVM support [(#131)](https://github.com/kevinheavey/solders/pull/131)
- Add support for UnsubscribeResult parsing [(#125)](https://github.com/kevinheavey/solders/pull/125).

### Changed

- Update maturin to 1.8.1 [(#130)](https://github.com/kevinheavey/solders/pull/130)

# [0.23.0] - 2024-11-10

### Changed

- Upgrade Solana Rust deps to 2.1 [(#123)](https://github.com/kevinheavey/solders/pull/123).
- Stop publishing Rust crates for now (most of the publish CI was failing anyway) [(#123)](https://github.com/kevinheavey/solders/pull/123).
- Drop support for bankrun on musllinux-i686 as its dependencies no longer support it [(#123)](https://github.com/kevinheavey/solders/pull/123).

## [0.22.0] - 2024-10-18

### Changed

- Add optional `token_program_id` param to `get_associated_token_address` [(#117)](https://github.com/kevinheavey/solders/pull/117).
- Upgrade Solana deps to 2.0 [(#116)](https://github.com/kevinheavey/solders/pull/116).
- Remove GetStakeActivationResp (no longer exists) [(#116)](https://github.com/kevinheavey/solders/pull/116).

### Fixed

- Avoid panic in `Keypair.from_base58_string` [(#93)](https://github.com/kevinheavey/solders/pull/93).
- Avoid panic in `Pubkey.create_program_address` [(#111)](https://github.com/kevinheavey/solders/pull/111).
- Avoid panic in `RPCError deserialization` [(#111)](https://github.com/kevinheavey/solders/pull/111).
- Add missing `stack_height` getter [(#103)](https://github.com/kevinheavey/solders/pull/103).

## [0.21.0] - 2024-03-13

### Changed

- Use pyo3 20.2 [(#81)](https://github.com/kevinheavey/solders/pull/81).
- Add back RPC modules on linux aarch64 [(#87)](https://github.com/kevinheavey/solders/pull/87)

## [0.20.0] - 2024-02-12

### Added

- Add address lookup table instructions and state [(#79)](https://github.com/kevinheavey/solders/pull/79)

### Changed

- Upgrade to Solana 1.18.1 [(#80)](https://github.com/kevinheavey/solders/pull/80). This also takes out the `ring` dependency (and all crates that use it) for linux-aarch64 builds.

## [0.19.0] - 2024-01-01

### Added

- Add `Keypair.from_seed_and_derivation_path` [(#75)](https://github.com/kevinheavey/solders/pull/75)
- Add Token Program ID (`solders.token.ID`)

### Fixed

- Fix (de)serialization of Account `owner` field [(#70)](https://github.com/kevinheavey/solders/pull/70)

### Changed

- Use PyO3 v0.19.2 [(#64)](https://github.com/kevinheavey/solders/pull/64)
- Upgrade to Solana 1.17.12 [(#71)](https://github.com/kevinheavey/solders/pull/71)

## [0.18.1] - 2023-06-03

### Changed

- Accept `Transaction | VersionedTransaction` in bankrun transaction methods [(#62)](https://github.com/kevinheavey/solders/pull/62)

## [0.18.0] - 2023-06-02

### Changed

- Use solana 1.16.0 [(#61)](https://github.com/kevinheavey/solders/pull/61)
- Rename `process_transaction_with_metadata` to just `process_transaction`.
  Remove `process_transaction_with_preflight` and the old `process_transaction`
  which had too many footguns. The new `process_transaction` uses the Rust
  `process_transaction_with_metadata` under the hood. [(#60)](https://github.com/kevinheavey/solders/pull/60)

### Fixed

- Remove `.string()` from `pubkey.pyi` as the method no longer exists [(#57)](https://github.com/kevinheavey/solders/pull/57)
- Fix `pre_token_balances` getter [(#59)](https://github.com/kevinheavey/solders/pull/59)

## [0.17.0] - 2023-05-11

### Added

Added partial support for the SPL Token Program [(#53)](https://github.com/kevinheavey/solders/pull/53)

## [0.16.0] - 2023-05-10

### Added

Added `bankrun.start_anchor()`

### Fixed

Fix type hint for `BanksClient.get_account`

## [0.15.1] - 2023-05-05

### Fixed

Fix type hint for `BanksClient.get_account`

## [0.15.0] - 2023-05-05

### Added

- Added `solders.bankrun` [(#47)](https://github.com/kevinheavey/solders/pull/47)
- Added `solders.compute_budget`

## [0.14.4] - 2023-02-22

### Added

Added `solders.message.to_bytes_versioned` and `from_bytes_versioned` to serialize versioned messages including the extra leading byte [(#45)](https://github.com/kevinheavey/solders/pull/45)

### Fixed

`transaction.Legacy` no longer implicitly casts to int when checking equality. This was breaking tx version checking when tx version was returned as `Legacy | int` [(#44)](https://github.com/kevinheavey/solders/pull/44)

## [0.14.3] - 2023-01-28

### Fixed 

Fix `MessageV0` JSON serialization [(#42)](https://github.com/kevinheavey/solders/pull/42)

## [0.14.2] - 2023-01-24

### Fixed

- Add `solders-primitives` to crates.io release flow.

## [0.14.1] - 2023-01-24

### Changed

- Use crates.io for the `pyo3` and `pythonize` dependencies [(#38)](https://github.com/kevinheavey/solders/pull/38)

## [0.14.0] - 2023-01-11

### Added

- Add `SimulateVersionedTransaction` [(#37)](https://github.com/kevinheavey/solders/pull/37)
- Support `VersionedMessage` in `GetFeeForMessage` [(#37)](https://github.com/kevinheavey/solders/pull/37)

## [0.13.0] - 2023-01-11

### Changed

Rename `SendTransaction` to `SendLegacyTransaction` [(#36)](https://github.com/kevinheavey/solders/pull/36)

### Added

Add `SendVersionedTransaction` [(#36)](https://github.com/kevinheavey/solders/pull/36)

## [0.12.0] - 2023-01-10

### Added

- Add `SendRawTransaction` [(#35)](https://github.com/kevinheavey/solders/pull/35)

## [0.11.0] - 2023-01-10

### Changed

- Move solders-macros into the monorepo [(#22)](https://github.com/kevinheavey/solders/pull/22)
- Don't leak custom error types in solders-traits; use ValueError instead [(#26)](https://github.com/kevinheavey/solders/pull/26)
- Improve macro hygiene [(#27)](https://github.com/kevinheavey/solders/pull/27) and [(#28)]([(#27)](https://github.com/kevinheavey/solders/pull/27))

### Added

- Extract solders-primitives into its own crate [(#24)](https://github.com/kevinheavey/solders/pull/24)
- Add EnumIntoPy derive macro [(#29)](https://github.com/kevinheavey/solders/pull/29)
- Add `common_methods_core` macro [(#30)](https://github.com/kevinheavey/solders/pull/30)
- Add `VersionedMessage` type alias [(#34)](https://github.com/kevinheavey/solders/pull/34)
- Make `signatures` writable for `VersionedTransaction` and `Transaction` [(#34)](https://github.com/kevinheavey/solders/pull/34)
- Add `from_legacy` to `VersionedTransaction` [(#34)](https://github.com/kevinheavey/solders/pull/34)
- Add `Signer` type alias [(#34)](https://github.com/kevinheavey/solders/pull/34)

### Fixed

- Fix incorrect field access in `max_transactions_per_entry` [(#34)](https://github.com/kevinheavey/solders/pull/34)

## [0.10.0] - 2022-10-31

### Changed

- Use `pythonize` for jsonParsed values [(#20)](https://github.com/kevinheavey/solders/pull/20)
- Extract `solders-traits` into its own crate [(#21)](https://github.com/kevinheavey/solders/pull/21)

## [0.9.3] - 2022-10-15

### Fixed

- Fix `TransactionError` parsing

## [0.9.2] - 2022-10-14

### Fixed

- Fix `InstructionError` parsing [(#19)](https://github.com/kevinheavey/solders/pull/19)

## [0.9.1] - 2022-10-14

### Fixed

- Remove incorrect `encoding` param from  `RpcSimulateTransactionConfig` [(#18)](https://github.com/kevinheavey/solders/pull/18)

## [0.9.0] - 2022-10-14

### Fixed

- Fix RPC error parsing and introduce new classes for RPC error messages [(#17)](https://github.com/kevinheavey/solders/pull/17)

## [0.8.1] - 2022-10-10

### Fixed

- Add missing getters to `UiTransactionStatusMeta` [(#16)](https://github.com/kevinheavey/solders/pull/16)


## [0.8.0] - 2022-10-10

### Changed

- `parse_websocket_message` now supports parsing an array of messages, and always returns a list [(#15)](https://github.com/kevinheavey/solders/pull/15)

### Fixed

- Add missing getters to `RpcBlockhash` [(#15)](https://github.com/kevinheavey/solders/pull/15)

## [0.7.0] - 2022-10-09

### Changed

- Replace `parse_<name>_maybe_json` funcs with `<name>MaybeJsonParsed` classes. Also fix bugs with parsing mixed responses. [(#14)](https://github.com/kevinheavey/
- Make `batch_from_json` pure Rust instead of relying on the Python `from_json` method. [(#14)](https://github.com/kevinheavey/solders/pull/14)

## [0.6.0] - 2022-10-05

### Added

- Add parsers for responses that may or may not be `jsonParsed` [(#13)](https://github.com/kevinheavey/solders/pull/13)

### Fixed

- Replace panic with SerdeJSONError when expecting JsonParsed data [(#13)](https://github.com/kevinheavey/solders/pull/13)

## [0.5.2] - 2022-10-01

### Fixed

- Fix incorrect alias name in type stubs [(#12)](https://github.com/kevinheavey/solders/pull/12)

## [0.5.1] - 2022-09-29

### Added

- Add ValidatorExit request and response [(#11)](https://github.com/kevinheavey/solders/pull/11)

## [0.5.0] - 2022-09-26

### Added

- Add RPC response parsing [(#10)](https://github.com/kevinheavey/solders/pull/10)
- Add versioned transactions [(#10)](https://github.com/kevinheavey/solders/pull/10)

## [0.4.0] - 2022-08-13

### Added

Add `Account` class [(#7)](https://github.com/kevinheavey/solders/pull/7)

### Fixed

Fix misspecified `typing_extensions` dependency [(#8)](https://github.com/kevinheavey/solders/pull/8)

## [0.3.1] - 2022-07-04

### Fixed

Make `rpc.requests.Body` alias available at runtime [(#6)](https://github.com/kevinheavey/solders/pull/6)

## [0.3.0] - 2022-07-04

### Added

- Added an RPC request builder under `solders.rpc.requests` [(#4)](https://github.com/kevinheavey/solders/pull/4)
  - Added related modules `solders.rpc.config`, `solders.rpc.filter`, `solders.account_decoder`, `solders.commitment_config` and `solders.transaction_status`.
  - Added JSON support to most classes.

## [0.2.0] - 2022-06-13

### Added

- Added a `from_bytes` constructor to every class that supports `__bytes__`
- Added pickle support [(#2)](https://github.com/kevinheavey/solders/pull/2)

## [0.1.4] - 2022-06-01

### Fixed

- Added missing `__richcmp__` for `MessageHeader`.
- Added missing `authorize_nonce_account` to `system_program`.

## [0.1.3] - 2022-05-30

### Fixed

- Added missing `TransactionError` export

## [0.1.2] - 2022-05-29

### Added

- Added docstrings to some error classes.

## [0.1.1] - 2022-05-28

### Fixed

- Fix some type hints.

## [0.1.0] - 2022-05-28

First release 🚀
