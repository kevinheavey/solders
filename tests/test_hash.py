from pytest import raises
from based58 import b58encode

from solders import Hash


def test_new_unique():
    assert Hash.new_unique() != Hash.new_unique()


def test_from_string():
    hashed = Hash.hash(bytes([1]))
    hash_base58_str = b58encode(bytes(hashed)).decode()
    assert Hash.from_string(hash_base58_str) == hashed
    too_big_str = hash_base58_str * 2
    with raises(ValueError) as excinfo:
        Hash.from_string(too_big_str)
    assert excinfo.value.args[0] == "string decoded to wrong size for hash"
    too_small_str = hash_base58_str[: len(hash_base58_str) // 2]
    with raises(ValueError) as excinfo:
        Hash.from_string(too_big_str)
    assert excinfo.value.args[0] == "string decoded to wrong size for hash"
    non_base58_str = "I" + hash_base58_str[1:]
    with raises(ValueError) as excinfo:
        Hash.from_string(non_base58_str)
    assert excinfo.value.args[0] == "failed to decoded string to hash"
