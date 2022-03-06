from based58 import b58encode, b58decode
from pytest import raises, fixture
from solders import Signature, Keypair, Pubkey


@fixture(scope="module")
def signature() -> Signature:
    return Keypair().sign_message(bytes([0]))


@fixture(scope="module")
def signature_base58_str(signature: Signature) -> str:
    return b58encode(bytes(signature)).decode()


def test_from_string(signature: Signature, signature_base58_str: str):
    assert Signature.from_string(signature_base58_str) == signature


def test_from_string_too_long(signature_base58_str: str):
    signature_base58_str_doubled = signature_base58_str * 2
    with raises(ValueError) as excinfo:
        Signature.from_string(signature_base58_str_doubled)
    assert excinfo.value.args[0] == "string decoded to wrong size for signature"


def test_from_string_too_short(signature_base58_str: str):
    with raises(ValueError) as excinfo:
        Signature.from_string(signature_base58_str[:4])
    assert excinfo.value.args[0] == "string decoded to wrong size for signature"


def test_from_string_non_base58(signature_base58_str: str):
    bad_str = "I" + signature_base58_str[1:]
    with raises(ValueError) as excinfo:
        Signature.from_string(bad_str)
    assert excinfo.value.args[0] == "failed to decode string to signature"


def test_off_curve_pubkey_verify_fails():
    # Golden point off the ed25519 curve
    off_curve_bytes = b58decode(b"9z5nJyQar1FUxVJxpBXzon6kHehbomeYiDaLi9WAMhCq")
    pubkey = Pubkey(off_curve_bytes)
    signature = Signature.default()
    assert not signature.verify(bytes(pubkey), bytes([0]))
