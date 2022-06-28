import pickle
from based58 import b58encode, b58decode
from pytest import raises, fixture
from solders.signature import Signature
from solders.keypair import Keypair
from solders.pubkey import Pubkey


@fixture(scope="module")
def signature() -> Signature:
    return Keypair().sign_message(bytes([0]))


@fixture(scope="module")
def signature_base58_str(signature: Signature) -> str:
    return str(signature)


def test_to_str(signature_base58_str: str, signature: Signature) -> None:
    assert signature_base58_str == b58encode(bytes(signature)).decode()


def test_from_string(signature: Signature, signature_base58_str: str) -> None:
    assert Signature.from_string(signature_base58_str) == signature


def test_from_string_too_long(signature_base58_str: str) -> None:
    signature_base58_str_doubled = signature_base58_str * 2
    with raises(ValueError) as excinfo:
        Signature.from_string(signature_base58_str_doubled)
    assert excinfo.value.args[0] == "string decoded to wrong size for signature"


def test_from_string_too_short(signature_base58_str: str) -> None:
    with raises(ValueError) as excinfo:
        Signature.from_string(signature_base58_str[:4])
    assert excinfo.value.args[0] == "string decoded to wrong size for signature"


def test_from_string_non_base58(signature_base58_str: str) -> None:
    bad_str = "I" + signature_base58_str[1:]
    with raises(ValueError) as excinfo:
        Signature.from_string(bad_str)
    assert excinfo.value.args[0] == "failed to decode string to signature"


def test_verify_valid() -> None:
    kp = Keypair()
    message = b"macaroni"
    sig = kp.sign_message(message)
    assert sig.verify(kp.pubkey(), message)


def test_verify_invalid() -> None:
    kp = Keypair()
    message = b"macaroni"
    assert not Signature.default().verify(kp.pubkey(), message)


def test_off_curve_pubkey_verify_fails() -> None:
    # Golden point off the ed25519 curve
    off_curve_bytes = b58decode(b"9z5nJyQar1FUxVJxpBXzon6kHehbomeYiDaLi9WAMhCq")
    pubkey = Pubkey(off_curve_bytes)
    signature = Signature.default()
    assert not signature.verify(pubkey, bytes([0]))


def test_to_bytes_array(signature: Signature) -> None:
    assert bytes(signature.to_bytes_array()) == bytes(signature)


def test_hash() -> None:
    msg = bytes([0])
    keypair = Keypair()
    sig = keypair.sign_message(msg)
    dupe = keypair.sign_message(msg)
    different = keypair.sign_message(bytes([1]))
    assert sig == dupe
    assert hash(sig) == hash(dupe)
    assert sig != different
    assert hash(sig) != hash(different)


def test_from_bytes() -> None:
    raw = b"123".rjust(Signature.LENGTH)
    assert Signature(raw) == Signature.from_bytes(raw)


def test_pickle() -> None:
    obj = Signature.default()
    assert pickle.loads(pickle.dumps(obj)) == obj


def test_json() -> None:
    obj = Signature.default()
    assert Signature.from_json(obj.to_json()) == obj
