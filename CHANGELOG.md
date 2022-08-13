# Changelog

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
