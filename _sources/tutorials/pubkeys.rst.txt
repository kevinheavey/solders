=======
Pubkeys
=======

The ``Pubkey`` class is used for both actual public keys,
and addresses that are off the ed25519 curve (e.g. program-derived addresses).

----------------------------------------
Checking if an address has a private key
----------------------------------------

Program-derived addresses (PDAs) do not lie on the ed25519 curve and thus do not have
private keys.

.. testcode::

   from solders.pubkey import Pubkey
   
   # Note that Keypair() will always give a public key that is valid for users
   key = Pubkey.from_string('5oNDL3swdJJF1g9DzJiZ4ynHXgszjAEpUkxVYejchzrY') # Valid public key
   assert key.is_on_curve() # Lies on the ed25519 curve and is suitable for users
   
   off_curve_address = Pubkey.from_string('4BJXYkfvg37zEmBbsacZjeQDpTNx91KppxFJxRqrz48e') # Valid public key
   assert not off_curve_address.is_on_curve() # Not on the ed25519 curve, therefore not suitable for users

---------------
Generating PDAs
---------------

The ``find_program_address`` static method takes an array of seed bytes and tries
adding a "bump" byte until it finds an off-curve address (i.e. a PDA):

.. testcode::

   from solders.pubkey import Pubkey
   
   program_id = Pubkey.from_string("G1DCNUQTSGHehwdLCAmRyAG8hf51eCHrLNUqkgGKYASj")
   pda, bump = Pubkey.find_program_address([b"test"], program_id)
   print(f"bump: {bump}; pda: {pda}")

.. testoutput::

   bump: 253; pda: AfEjen5hHkTkEqy2yfyPhDQWq7dc7zbWU2aJmB3h8brU
