# Changelog

## [0.3.0] - 2022-07-04

### Added

- Added an RPC request builder under `solders.rpc.requests` [(#4)](https://github.com/kevinheavey/solders/pull/4)

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
