from pytest import raises, mark
from solder import is_on_curve, PublicKey

on_curve_data = [
    (
        b"\xc1M\xce\x1e\xa4\x86<\xf1\xbc\xfc\x12\xf4\xf2\xe2Y"
        b"\xf4\x8d\xe4V\xb7\xf9\xd4\\!{\x04\x89j\x1f\xfeA\xdc",
        True,
    ),
    (
        b"6\x8d-\x96\xcf\xe7\x93G~\xe0\x17r\\\x9c%\x9a\xab\xa6"
        b"\xa9\xede\x02\xbf\x83=\x10,P\xfbh\x8ev",
        True,
    ),
    (
        b"\x00y\xf0\x82\xa6\x1c\xc7N\xa5\xe2\xab\xedd\xbb\xf7_2"
        b"\xfb\xddSz\xff\xf7RW\xedg\x16\xc9\xe3r\x99",
        False,
    ),
]


@mark.parametrize("test_input,expected", on_curve_data)
def test_is_on_curve(benchmark, test_input, expected):
    result = benchmark(is_on_curve, test_input)
    assert result is expected


@mark.parametrize("test_input,expected", on_curve_data)
def test_is_on_curve_method(test_input, expected):
    pubkey = PublicKey(test_input)
    result = pubkey.is_on_curve()
    assert result is expected


def test_is_on_curve_wrong_length():
    data = b"\xc1M"
    with raises(BaseException):
        is_on_curve(data)
