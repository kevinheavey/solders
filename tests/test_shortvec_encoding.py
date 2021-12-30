from typing import List
from solder import encode_length


def assert_encoded_array(
    buffer: bytearray, length: int, prev_length: int, expected: List[int]
) -> None:
    """Helper to encode length of an array."""
    assert len(buffer) == prev_length
    actual = encode_length(length)
    buffer.extend(actual)
    assert len(buffer) == prev_length + len(expected)
    assert bytes(buffer[-len(expected) :]) == bytes(expected)  # noqa: 203


def test_encode_length():
    """Test encode length."""
    buffer = bytearray()
    prev_length = 0

    expected = [0]
    assert_encoded_array(buffer, 0, prev_length, expected)
    prev_length += len(expected)

    expected = [5]
    assert_encoded_array(buffer, 5, prev_length, expected)
    prev_length += len(expected)

    expected = [0x7F]
    assert_encoded_array(buffer, 0x7F, prev_length, expected)
    prev_length += len(expected)

    expected = [0x80, 0x1]
    assert_encoded_array(buffer, 0x80, prev_length, expected)
    prev_length += len(expected)

    expected = [0xFF, 0x1]
    assert_encoded_array(buffer, 0xFF, prev_length, expected)
    prev_length += len(expected)

    expected = [0x80, 0x2]
    assert_encoded_array(buffer, 0x100, prev_length, expected)
    prev_length += len(expected)

    expected = [0xFF, 0xFF, 0x1]
    assert_encoded_array(buffer, 0x7FFF, prev_length, expected)
    prev_length += len(expected)

    assert prev_length == len(buffer) == 12
