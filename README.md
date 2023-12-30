<div align="center">
    <img src="https://raw.githubusercontent.com/kevinheavey/solders/main/docs/logo.jpeg" width="50%" height="50%">
</div>

---

[![PyPI version](https://badge.fury.io/py/solders.svg)](https://badge.fury.io/py/solders)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/kevinheavey/solders/blob/main/LICENSE)

# Solders

`solders` is a high-performance Python toolkit for Solana, written in Rust. It provides robust solutions to the following problems:

- Core SDK stuff: keypairs, pubkeys, signing and serializing transactions - that sort of thing.
- RPC stuff: building requests and parsing responses (no networking stuff - if you want help with that, 
[solana-py](https://michaelhly.github.io/solana-py/rpc/async_api/) is your friend).
- Integration testing stuff: the `solders.bankrun` module is an alternative to `solana-test-validator` that's much more convenient and **much** faster. It's based on [solana-program-test](https://crates.io/crates/solana-program-test) if you know that is.

## What about solana-py?

`solders` and `solana-py` are good friends. `solana-py` uses `solders` under the hood extensively in its
core API and RPC API. The main differences are:

- `solders` doesn't have functions to actually interact with the RPC server (though `solana-py` does use the RPC code from `solders`).
- `solders` doesn't provide SPL Token and SPL Memo clients.
- `solana-py` may not have support for all the RPC requests and responses provided by `solders`.
- `solana-py` doesn't have anything like the `bankrun` testing kit.

Since `solana-py` uses `solders` under the hood and they don't duplicate each other's features, you should just use whichever library you need.

## Installation

```
pip install solders
```

Note: Requires Python >= 3.7.

## Example Usage

```python
>>> from solders.message import Message
>>> from solders.keypair import Keypair
>>> from solders.instruction import Instruction
>>> from solders.hash import Hash
>>> from solders.transaction import Transaction
>>> from solders.pubkey import Pubkey
>>> program_id = Pubkey.default()
>>> arbitrary_instruction_data = bytes([1])
>>> accounts = []
>>> instruction = Instruction(program_id, arbitrary_instruction_data, accounts)
>>> payer = Keypair()
>>> message = Message([instruction], payer.pubkey())
>>> blockhash = Hash.default()  # replace with a real blockhash
>>> tx = Transaction([payer], message, blockhash)

```

## Development

### Setup

1. Install [poetry](https://python-poetry.org/)
2. Install dev dependencies:

```
poetry install
```

3. Activate the poetry shell:

```sh
poetry shell
```

### Testing

1. Run `maturin develop` to compile the Rust code.
2. Run `make fmt`, `make lint`, and `make test`.
