from operator import ge, gt, le, lt
import pickle
from typing import Callable, Any
from pytest import raises, mark

from pybip39 import Mnemonic, Seed
from solders.keypair import Keypair


def test_from_bytes() -> None:
    raw_bytes = (
        b"\x99\xda\x95Y\xe1^\x91>\xe9\xab.S\xe3\xdf\xadW]\xa3;I\xbe\x11%\xbb\x92.3IOI"
        b'\x88(\x1b/I\tn>]\xbd\x0f\xcf\xa9\xc0\xc0\xcd\x92\xd9\xab;!TK4\xd5\xddJe\xd9\x8b\x87\x8b\x99"'
    )
    kp = Keypair.from_bytes(raw_bytes)
    assert bytes(kp) == raw_bytes


def test_to_bytes_array() -> None:
    kp = Keypair()
    assert kp.to_bytes_array() == list(bytes(kp))


def test_str() -> None:
    expected = (
        "2AXDGYSE4f2sz7tvMMzyHvUfcoJmxudvdhBcmiUSo6ij"
        "wfYmfZYsKRxboQMPh3R4kUhXRVdtSXFXMheka4Rc4P2"
    )
    kp = Keypair.from_bytes([1] * 64)
    assert str(kp) == expected
    assert Keypair.from_base58_string(expected) == kp


def test_sign_message() -> None:
    seed = bytes([1] * 32)
    keypair = Keypair.from_seed(seed)
    msg = b"hello"
    expected_hex = (
        "e1430c6ebd0d53573b5c803452174f8991ef5955e0906a09e8"
        "fdc7310459e9c82a402526748c3431fe7f0e5faafbf7e703234"
        "789734063ee42be17af16438d08"
    )
    signature = keypair.sign_message(msg)
    assert bytes(signature).hex() == expected_hex


@mark.parametrize("test_input", [bytes(0), bytes(1), bytes(65)])
def test_from_bytes_wrong_size(test_input: bytes) -> None:
    with raises(ValueError) as excinfo:
        Keypair.from_bytes(test_input)
    test_input_len = len(test_input)
    assert (
        excinfo.value.args[0]
        == f"expected a sequence of length 64 (got {test_input_len})"
    )


def test_from_bytes_invalid_input() -> None:
    with raises(ValueError) as excinfo:
        Keypair.from_bytes(b"a" * 64)
    assert excinfo.value.args[0] == "signature error: Cannot decompress Edwards point"


def test_set_operations() -> None:
    """Tests that a keypair is now hashable with the appropriate set operations."""
    keypair_primary = Keypair()
    keypair_secondary = Keypair()
    keypair_duplicate = keypair_secondary
    keypair_set = {keypair_primary, keypair_secondary, keypair_duplicate}
    assert hash(keypair_primary) != hash(keypair_secondary)
    assert hash(keypair_secondary) == hash(keypair_duplicate)
    assert len(keypair_set) == 2


def test_equal() -> None:
    assert Keypair.from_seed(bytes([0] * 32)) == Keypair.from_seed(bytes([0] * 32))


def test_not_equal() -> None:
    assert Keypair.from_seed(bytes([0] * 32)) != Keypair.from_seed(bytes([1] * 32))


@mark.parametrize("op", [ge, gt, le, lt])
def test_ordering_raises(op: Callable[[Any, Any], bool]) -> None:
    kp1 = Keypair()
    kp2 = Keypair()
    with raises(TypeError):
        op(kp1, kp2)


def test_from_seed() -> None:
    good_seed = bytes([0] * 32)
    kp = Keypair.from_seed(good_seed)
    assert kp.secret() == good_seed
    too_short_seed = bytes([0] * 31)
    with raises(ValueError) as excinfo:
        Keypair.from_seed(too_short_seed)
    assert excinfo.value.args[0] == "expected a sequence of length 32 (got 31)"


def test_from_seed_phrase_and_passphrase() -> None:
    mnemonic = Mnemonic()
    passphrase = "42"  # noqa: S105
    seed = Seed(mnemonic, passphrase)
    expected_keypair = Keypair.from_seed(bytes(seed)[:32])
    keypair = Keypair.from_seed_phrase_and_passphrase(mnemonic.phrase, passphrase)
    assert keypair.pubkey() == expected_keypair.pubkey()


def test_pickle() -> None:
    obj = Keypair()
    assert pickle.loads(pickle.dumps(obj)) == obj
