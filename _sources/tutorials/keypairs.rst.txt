========
Keypairs
========

The ``Keypair`` class is a wrapper around a 64-byte array, where
the first 32 bytes consist of your secret seed and the latter 32 bytes
are your pubkey.

------------------------
Generating a new keypair
------------------------

Often you need to generate a new keypair on the fly:

.. testcode::

   from solders.keypair import Keypair
   keypair = Keypair()

---------------------------------
Converting a keypair to raw bytes
---------------------------------

To get the raw bytes of a keypair object you just call
``bytes(keypair)``:

.. testcode::

   from solders.keypair import Keypair
   keypair = Keypair()
   raw = bytes(keypair)

---------------------------------
Restoring a keypair from a secret
---------------------------------

If you already have the 64-byte secret key,
you can use ``Keypair.from_bytes``:

.. testcode::

   from solders.keypair import Keypair
   
   secret_key = [
       174, 47, 154, 16, 202, 193, 206, 113,
       199, 190, 53, 133, 169, 175, 31, 56,
       222, 53, 138, 189, 224, 216, 117, 173,
       10, 149, 53, 45, 73, 251, 237, 246,
       15, 185, 186, 82, 177, 240, 148, 69,
       241, 227, 167, 80, 141, 89, 240, 121,
       121, 35, 172, 247, 68, 251, 226, 218,
       48, 63, 176, 109, 168, 89, 238, 135,
   ]
   
   keypair = Keypair.from_bytes(secret_key)
   print(f"Created Keypair with public key: {keypair.pubkey()}")

.. testoutput::
   :options: -ELLIPSIS, +NORMALIZE_WHITESPACE

   Created Keypair with public key: 24PNhTaNtomHhoy3fTRaMhAFCRj4uHqhZEEoWrKDbR5p

This is the inverse operation to ``bytes(keypair)``.

If the secret is in base58 format, you can use ``Keypair.from_base58_string``:

.. testcode::

   from solders.keypair import Keypair

   b58_string = "5MaiiCavjCmn9Hs1o3eznqDEhRwxo7pXiAYez7keQUviUkauRiTMD8DrESdrNjN8zd9mTmVhRvBJeg5vhyvgrAhG"
   keypair = Keypair.from_base58_string(b58_string)
   print(f"Created Keypair with public key: {keypair.pubkey()}")

.. testoutput::
   :options: -ELLIPSIS, +NORMALIZE_WHITESPACE

   Created Keypair with public key: 5pVyoAeURQHNMVU7DmfMHvCDNmTEYXWfEwc136GYhTKG

-------------------
Verifying a keypair
-------------------

If you have a keypair and a pubkey, you can check whether the given pubkey
comes from that keypair:

.. testcode::

   from solders.keypair import Keypair
   from solders.pubkey import Pubkey
   
   public_key = Pubkey.from_string("24PNhTaNtomHhoy3fTRaMhAFCRj4uHqhZEEoWrKDbR5p")
   
   keys = [
           174, 47, 154, 16, 202, 193, 206, 113, 199, 190, 53, 133, 169, 175, 31, 56, 222, 53, 138,
           189, 224, 216, 117, 173, 10, 149, 53, 45, 73, 251, 237, 246, 15, 185, 186, 82, 177, 240,
           148, 69, 241, 227, 167, 80, 141, 89, 240, 121, 121, 35, 172, 247, 68, 251, 226, 218, 48,
           63, 176, 109, 168, 89, 238, 135,
       ]
   keypair = Keypair.from_bytes(keys)

   assert keypair.pubkey() == public_key

-------------------------------------------------
Restoring a keypair from a mnemonic (seed phrase)
-------------------------------------------------

::

   from solders.keypair import Keypair
   from mnemonic import Mnemonic
   
   mnemo = Mnemonic("english")
   seed = mnemo.to_seed("pill tomorrow foster begin walnut borrow virtual kick shift mutual shoe scatter")
   keypair = Keypair.from_seed(seed[:32])

------------------------------
Signing and verifying messages
------------------------------

The primary function of a keypair is to sign messages and enable verification of the signature.
Verification of a signature allows the recipient to be sure that the data was signed by the owner of a specific private key.

.. testcode::

   from solders.keypair import Keypair
   
   secret_key = [
         174, 47, 154, 16, 202, 193, 206, 113, 199, 190, 53, 133, 169, 175, 31, 56, 222, 53, 138, 189, 224, 216, 117,
         173, 10, 149, 53, 45, 73, 251, 237, 246, 15, 185, 186, 82, 177, 240, 148, 69, 241, 227, 167, 80, 141, 89, 240,
         121, 121, 35, 172, 247, 68, 251, 226, 218, 48, 63, 176, 109, 168, 89, 238, 135,
   ] 
   keypair = Keypair.from_bytes(secret_key)
   message = b"The quick brown fox jumps over the lazy dog"
   signature = keypair.sign_message(message)
   assert signature.verify(keypair.pubkey(), message)
