=======
Bankrun
=======

The ``bankrun`` module offers a quick and powerful yet lightweight solution for testing Solana programs.
While people often use ``solana-test-validator`` for this,
``bankrun`` is orders of magnitude faster and far more convenient.
You don't have to
take care of an external process and you can start as many ``bankrun``
instances as you like without worrying about ports in use or hogging your machines resources.

You can also do things that are not possible with ``solana-test-validator``,
such as jumping back and forth in time or dynamically setting account data.

If you've used `solana-program-test <https://crates.io/crates/solana-program-test>`_
you'll be familiar with ``bankrun``, since that's what it uses under the hood.

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
we want to test our own programs. ``solana-program-test`` is a bit fussy about
how this is done.

Firstly, the program's ``.so`` file must be present in one of the following directories:

* ``./tests/fixtures`` (just create this directory if it doesn't exist)
* The current working directory
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

------------------
Anchor integration
------------------

If you have an Anchor workspace, ``bankrun`` can make some extra assumptions that make it more
convenient to get started. Just use ``start_anchor`` and give it the path to the project root
(the folder containing the ``Anchor.toml`` file). The programs in the workspace will be automatically
deployed to the test environment.

Example:

.. literalinclude:: ../../tests/bankrun/test_anchor.py

-----------
Time travel
-----------

Many programs rely on the ``Clock`` sysvar: for example, a mint that doesn't become available until after
a certain time. With ``bankrun`` you can dynamically overwrite the ``Clock`` sysvar using ``context.set_clock()``.
Here's an example using a program that panics if ``clock.unix_timestamp`` is greater than 100
(which is on January 1st 1970):

.. literalinclude:: ../../tests/bankrun/clock-example/test_clock_example.py

See also: ``context.warp_to_slot()``, which lets you jump to a future slot.

--------------------------
Writing arbitrary accounts
--------------------------

Bankrun lets you write any account data you want, regardless of
whether the account state would even be possible.

Here's an example where we give an account a bunch of USDC,
even though we don't have the USDC mint keypair. This is
convenient for testing because it means we don't have to
work with fake USDC in our tests:

.. literalinclude:: ../../tests/bankrun/test_usdc_mint.py

.. tip::
   If you want to set account data *after* calling ``bankrun.start()``,
   you can use ``context.set_account()``.

--------------
Other features
--------------

Other things you can do with ``bankrun`` include:

* Changing the max compute units with the ``compute_max_units`` parameter.
* Changing the transaction account lock limit with the ``transaction_account_lock_limit`` parameter.

--------------------------------------------
When should I use ``solana-test-validator``?
--------------------------------------------

While ``bankrun`` is faster and more convenient, it is also less like a real RPC node.
So ``solana-test-validator`` is still useful when you need to call RPC methods that ``BanksServer``
doesn't support, or when you want to test something that depends on real-life validator behaviour
rather than just testing your program and client code.

In general though I would recommend using ``bankrun`` wherever possible, as it will make your life
much easier.

-------------------
Supported platforms
-------------------

``bankrun`` is not included on ``windows`` and ``musllinux-i686`` targets, but otherwise
should run everywhere that ``solders`` runs.
