from solders.null_signer import NullSigner
from solders.signature import Signature
from solders.keypair import Keypair


def test_null_signer():
    msg = b"hi"
    pubkey = Keypair().pubkey()
    assert NullSigner(pubkey).sign_message(msg) == Signature.default()
