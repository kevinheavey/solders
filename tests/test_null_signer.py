from solders.null_signer import NullSigner
from solders.signature import Signature
from solders.keypair import Keypair


def test_null_signer() -> None:
    msg = b"hi"
    pubkey = Keypair().pubkey()
    ns = NullSigner(pubkey)
    assert ns.sign_message(msg) == Signature.default()
    assert NullSigner.from_bytes(bytes(ns)) == ns
