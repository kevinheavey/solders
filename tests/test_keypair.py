from pytest import raises, mark
from mnemonic import Mnemonic
from solder import Keypair, Pubkey


def test_new():
    keypair = Keypair()
    assert len(bytes(keypair.pubkey())) == Pubkey.LENGTH


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


def test_from_seed() -> None:
    keypair = Keypair.from_seed(bytes([0] * 32))
    assert bytes(keypair.secret()) == bytes([0] * 32)


def test_from_seed_phrase_and_passphrase() -> None:
    mnemo = Mnemonic("english")
