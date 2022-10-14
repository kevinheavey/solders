# Changelog

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
