from pytest import raises
from solders import Keypair, Presigner


def test_presigner() -> None:
    keypair = Keypair.from_seed(bytes([0] * 32))
    pubkey = keypair.pubkey()
    data = bytes([1])
    sig = keypair.sign_message(data)

    # Signer
    presigner = Presigner(pubkey, sig)
    assert presigner.pubkey() == pubkey
    assert presigner.sign_message(data) == sig
    bad_data = bytes([2])
    with raises(ValueError) as excinfo:
        presigner.sign_message(bad_data)
    assert excinfo.value.args[0] == "presigner error"

    # PartialEq
    assert presigner == keypair
    assert keypair == presigner
    presigner2 = Presigner(pubkey, sig)
    assert presigner == presigner2
