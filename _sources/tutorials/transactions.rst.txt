============
Transactions
============

Solana has two kinds of transactions: "legacy" transactions represented by
the ``Transaction`` class and versioned transactions represented by the
``VersionedTransaction`` class. These examples will focus on 
versioned transactions which are what you should use if you have a choice.

These examples do not demonstrate sending transactions or fetching the latest blockhash,
which you can do with the ``send_transaction`` and ``get_latest_blockhash`` methods in
solana-py.

-----------
Sending SOL
-----------

Here we construct a transaction with one instruction - it sends SOL from one
wallet to another via the System Program:

.. testcode::

   from solders.hash import Hash
   from solders.keypair import Keypair
   from solders.message import MessageV0
   from solders.system_program import TransferParams, transfer
   from solders.transaction import VersionedTransaction
   
   sender = Keypair()  # let's pretend this account actually has SOL to send
   receiver = Keypair()
   ix = transfer(
       TransferParams(
           from_pubkey=sender.pubkey(), to_pubkey=receiver.pubkey(), lamports=1_000_000
       )
   )
   blockhash = Hash.default()  # replace with a real blockhash using get_latest_blockhash
   msg = MessageV0.try_compile(
       payer=sender.pubkey(),
       instructions=[ix],
       address_lookup_table_accounts=[],
       recent_blockhash=blockhash,
   )
   tx = VersionedTransaction(msg, [sender])

---------------
Partial signing
---------------

Suppose you have a transaction that both Alice and Bob need to sign, and Bob doesn't want to give
Alice his keypair because last time he did that all his apes got stolen.

One solution is for Alice to create a transaction containing her signature and a dummy signature using
the ``NullSigner`` class. She then serializes this transaction and sends it to Bob, who deserializes it
and replaces the dummy signature with his own signature:

.. testcode::

   from solders.hash import Hash
   from solders.instruction import AccountMeta, Instruction
   from solders.keypair import Keypair
   from solders.message import MessageV0, to_bytes_versioned
   from solders.null_signer import NullSigner
   from solders.pubkey import Pubkey
   from solders.transaction import VersionedTransaction
   
   keypair0 = Keypair()
   keypair1 = Keypair()
   ix = Instruction(
       Pubkey.new_unique(), b"", [AccountMeta(keypair1.pubkey(), True, False)]
   )
   message = MessageV0.try_compile(keypair0.pubkey(), [ix], [], Hash.default())
   # sign with a real signer and a null signer
   signers = (keypair0, NullSigner(keypair1.pubkey()))
   partially_signed = VersionedTransaction(message, signers)
   serialized = bytes(partially_signed)
   deserialized = VersionedTransaction.from_bytes(serialized)
   assert deserialized == partially_signed
   deserialized_message = deserialized.message
   # find the null signer in the deserialized transaction
   keypair1_sig_index = next(
       i
       for i, key in enumerate(deserialized_message.account_keys)
       if key == keypair1.pubkey()
   )
   sigs = deserialized.signatures
   # replace the null signature with a real signature
   sigs[keypair1_sig_index] = keypair1.sign_message(
       to_bytes_versioned(deserialized_message)
   )
   deserialized.signatures = sigs
   fully_signed = VersionedTransaction(message, [keypair0, keypair1])
   assert deserialized.signatures == fully_signed.signatures
   assert deserialized == fully_signed
   assert bytes(deserialized) == bytes(fully_signed)
