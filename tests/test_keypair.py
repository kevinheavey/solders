from pytest import raises, mark
from solder import Keypair


def test_new():
    Keypair()


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


def test_to_bytes():
    print(Keypair().to_bytes())


def test_bytes():
    print(bytes(Keypair()))
