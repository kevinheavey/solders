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

    >>> from solders.pubkey import Pubkey
    >>> Pubkey.default()
    Pubkey(
        11111111111111111111111111111111,
    )



.. toctree::
   :maxdepth: 3
   :caption: Contents:

   self
   api_reference/index