from operator import ge, gt, le, lt
from typing import Callable
from pytest import raises, mark

from pybip39 import Mnemonic, Seed
from solders import Keypair, Pubkey


def test_from_bytes():
    raw_bytes = (
        b"\x99\xda\x95Y\xe1^\x91>\xe9\xab.S\xe3\xdf\xadW]\xa3;I\xbe\x11%\xbb\x92.3IOI"
        b'\x88(\x1b/I\tn>]\xbd\x0f\xcf\xa9\xc0\xc0\xcd\x92\xd9\xab;!TK4\xd5\xddJe\xd9\x8b\x87\x8b\x99"'
    )
    Keypair.from_bytes(raw_bytes)


@mark.parametrize("test_input", [bytes(0), bytes(1), bytes(65)])
def test_from_bytes_wrong_size(test_input: bytes):
    with raises(ValueError) as excinfo:
        Keypair.from_bytes(test_input)
    assert (
        excinfo.value.args[0] == "signature error: Keypair must be 64 bytes in length"
    )


def test_from_bytes_invalid_input():
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
def test_ordering_raises(op: Callable) -> None:
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
    assert excinfo.value.args[0] == "Seed is too short"


def test_from_seed_phrase_and_passphrase() -> None:
    mnemonic = Mnemonic()
    passphrase = "42"
    seed = Seed(mnemonic, passphrase)
    expected_keypair = Keypair.from_seed(bytes(seed))
    keypair = Keypair.from_seed_phrase_and_passphrase(mnemonic.phrase, passphrase)
    assert keypair.pubkey() == expected_keypair.pubkey()
