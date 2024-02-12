# Changelog

## [0.20.0] - 2024-02-12

### Added

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
