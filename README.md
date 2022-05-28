<div align="center">
    <img src="https://raw.githubusercontent.com/kevinheavey/solders/main/docs/logo.jpeg" width="50%" height="50%">
</div>

---

[![Actions
Status](https://github.com/kevinheavey/solders/workflows/CI/badge.svg)](https://github.com/kevinheavey/solders/actions?query=workflow%3ACI)
[![PyPI version](https://badge.fury.io/py/solders.svg)](https://badge.fury.io/py/solders)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/kevinheavey/solders/blob/maim/LICENSE)
[![Code style: black](https://img.shields.io/badge/code%20style-black-000000.svg)](https://github.com/psf/black)

# Solders

`solders` is a Python binding to the
[Solana Rust SDK](https://docs.rs/solana-sdk/latest/solana_sdk/).
It provides robust, high-performance solutions to core Solana tasks such as transaction signing and serialization, and saves us from reimplementing Solana logic in pure Python.

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
