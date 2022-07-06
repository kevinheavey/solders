.. image:: logo.jpeg

Solders
=======

``solders`` is a Python binding to the
`Solana Rust SDK <https://docs.rs/solana-sdk/latest/solana_sdk/>`_.
It provides robust, high-performance solutions to core Solana tasks such as transaction signing and serialization, and saves us from reimplementing Solana logic in pure Python.

Installation
^^^^^^^^^^^^

::

    pip install solders

.. note:: Requires Python >= 3.7.

Example Usage
^^^^^^^^^^^^^

::

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



.. toctree::
   :maxdepth: 3
   :caption: Contents:

   self
   api_reference/index