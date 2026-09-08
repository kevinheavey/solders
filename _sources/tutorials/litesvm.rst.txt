=======
LiteSVM
=======

``solders.litesvm`` is the Python wrapper for `LiteSVM <https://github.com/LiteSVM/litesvm>`_. It brings best-in-class Solana testing
to Python, giving you a powerful, fast and ergonomic way to test Solana programs.

For a standard testing workflow, LiteSVM offers an experience superior to ``solana-test-validator`` (slow, unwieldy)
and the old ``solders.bankrun`` module (reasonably fast and powerful, but inherits a lot of warts from ``solana-program-test``).

---------------
Minimal example
---------------

This example just transfers lamports from Alice to Bob without loading
any programs of our own.

.. literalinclude:: ../../tests/litesvm/test_one_transfer_litesvm.py

Note: by default the ``LiteSVM`` instance includes some core programs such as
the System Program and SPL Token.

------------------
Deploying programs
------------------

Most of the time we want to do more than just mess around with token transfers - 
we want to test our own programs.

.. tip::
    If you want to pull a Solana program from mainnet or devnet, use the ``solana program dump`` command from the Solana CLI.

To add a compiled program to our tests we can use the ``add_program_from_file`` method.

Here's an example using a [simple program](https://github.com/solana-labs/solana-program-library/tree/bd216c8103cd8eb9f5f32e742973e7afb52f3b81/examples/rust/logging)
from the Solana Program Library that just does some logging:

.. literalinclude:: ../../tests/litesvm/test_spl_logging_litesvm.py

-----------
Time travel
-----------

Many programs rely on the ``Clock`` sysvar: for example, a mint that doesn't become available until after
a certain time. With ``litesvm`` you can dynamically overwrite the ``Clock`` sysvar using ``svm.set_clock()``.
Here's an example using a program that panics if ``clock.unix_timestamp`` is greater than 100
(which is on January 1st 1970):

.. literalinclude:: ../../tests/litesvm/clock-example/test_clock_example_litesvm.py

See also: ``svm.warp_to_slot()``, which lets you jump to a future slot.

--------------------------
Writing arbitrary accounts
--------------------------

LiteSVM lets you write any account data you want, regardless of
whether the account state would even be possible.

Here's an example where we give an account a bunch of USDC,
even though we don't have the USDC mint keypair. This is
convenient for testing because it means we don't have to
work with fake USDC in our tests:

.. literalinclude:: ../../tests/litesvm/test_usdc_mint_litesvm.py

----------------------------------------
Copying Accounts from a live environment
----------------------------------------

If you want to copy accounts from mainnet or devnet, you can use the ``solana account`` command in the Solana CLI to save account data to a file.

--------------
Other features
--------------

Other things you can do with ``litesvm`` include:

* Changing the max compute units and other compute budget behaviour using the ``with_compute_budget`` method.
* Disable transaction signature checking using ``svm.with_sigverify(false)``.
* Find previous transactions using the ``get_transaction`` method.

--------------------------------------------
When should I use ``solana-test-validator``?
--------------------------------------------

While ``litesvm`` is faster and more convenient, it is also less like a real RPC node.
So ``solana-test-validator`` is still useful when you need to call RPC methods that LiteSVM
doesn't support, or when you want to test something that depends on real-life validator behaviour
rather than just testing your program and client code.

In general though it is recommended to use ``litesvm`` wherever possible, as it will make your life
much easier.

-------------------
Supported platforms
-------------------

``litesvm`` is supported on Linux x64 and MacOS targets. If you find a platform that is not supported
but which can run the ``litesvm`` Rust crate, please open an issue.
