=======
Bankrun
=======

The ``bankrun`` module offers a quick, lightweight solution for integration tests on Solana programs.
While people often use ``solana-test-validator`` for this,
``bankrun`` is orders of magnitude faster and far more convenient.
You don't have to
take care of an external process and you can start as many ``bankrun``
instances as you like without worrying about ports in use or hogging your machines resources.

If you've used `solana-program-test <https://crates.io/crates/solana-program-test>`_
you'll be familiar with ``bankrun``, since that's what it uses under-the-hood.

For those unfamiliar, ``bankrun`` and ``solana-program-test`` work by spinning up a lightweight
``BanksServer`` that's like an RPC but much faster, and creating a ``BanksClient`` to talk to the
server. This author thought ``solana-program-test`` was a boring name, so he chose ``bankrun`` instead
(you're running Solana `Banks <https://github.com/solana-labs/solana/blob/master/runtime/src/bank.rs>`_).


---------------
Minimal example
---------------

This example just transfers lamports from Alice to Bob without loading
any programs of our own. Note: you'll need ``pytest-asyncio`` installed in
the environment, since the test function is async.

.. literalinclude:: ../../tests/bankrun/test_one_transfer.py

Some things to note here:

* The ``context`` object contains a ``banks_client`` to talk to the ``BanksServer``,
  a ``payer`` keypair that has been funded with a bunch of SOL, and a ``last_blockhash``
  that we can use in our transactions.
* We haven't loaded any specific programs, but by default we have access to
  the System Program, the SPL token programs and the SPL memo program.

------------------
Deploying programs
------------------

Most of the time we want to do more than just mess around with token transfers - 
we want to test our own programs.``solana-program-test`` is a bit fussy about
how this is done.

Firstly, the program's ``.so`` file must be present in one of the following directories:

* ``./tests/fixtures`` (just create this directory if it doesn't exist)
* A directory you define in the ``BPF_OUT_DIR`` or ``SBF_OUT_DIR`` environment variables.

(If you're not aware, the ``.so`` file is created when you run ``anchor build`` or ``cargo build-sbf``
and can be found in ``target/deploy``).

Now to add the program to our tests we use the ``programs`` parameter in the ``start`` function.
The program name used in this parameter must match the filename without the ``.so`` extension.

Here's an example using a `simple program <https://github.com/solana-labs/solana-program-library/tree/bd216c8103cd8eb9f5f32e742973e7afb52f3b81/examples/rust/logging>`_
from the Solana monorepo that just does some logging:

.. literalinclude:: ../../tests/bankrun/test_spl_logging.py

The ``.so`` file must be named ``spl_example_logging.so``, since ``spl_example_logging`` is
the name we used in the ``programs`` parameter.

--------------
Other features
--------------

Other things you can do with ``bankrun`` include:

* Adding arbitrary account data with the ``accounts`` parameter.
* Changing the max compute units with the ``compute_max_units`` parameter.
* Jumping to a future slot with ``context.warp_to_slot()``.

--------------------------------------------
When should I use ``solana-test-validator``?
--------------------------------------------

While ``bankrun`` is faster and more convenient, it is also less like a real RPC node.
So ``solana-test-validator`` is still useful when you need to call RPC methods that ``BanksServer``
doesn't support, or when you want to test something that depends on specific validator behaviour
rather than just testing your program and client code.

-------------------
Supported platforms
-------------------

``bankrun`` is not included on ``windows`` and ``musllinux-i686`` targets, but otherwise
should run everywhere that ``solders`` runs.
