from pytest import raises, mark
from based58 import b58encode

from solders.hash import Hash, ParseHashError

HASHED = Hash.hash(bytes([1]))
HASH_BASE58_STR = b58encode(bytes(HASHED)).decode()


def test_new_unique() -> None:
    assert Hash.new_unique() != Hash.new_unique()


def test_from_string() -> None:
    assert Hash.from_string(HASH_BASE58_STR) == HASHED


@mark.parametrize(
    "test_input,expected_err",
    [
        (
            HASH_BASE58_STR * 2,
            "string decoded to wrong size for hash",
        ),
        (
            HASH_BASE58_STR[: len(HASH_BASE58_STR) // 2],
            "string decoded to wrong size for hash",
        ),
        ("I" + HASH_BASE58_STR[1:], "failed to decoded string to hash"),
    ],
)
def test_from_string_error(test_input: str, expected_err: str) -> None:
    with raises(ParseHashError) as excinfo:
        Hash.from_string(test_input)
    assert excinfo.value.args[0] == expected_err


def test_from_bytes() -> None:
    raw = b"123"
    assert Hash.from_bytes(raw) == Hash(raw)


def test_hashable() -> None:
    assert isinstance(hash(Hash.default()), int)
