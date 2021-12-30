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
def test_is_on_curve(test_input, expected):
    result = is_on_curve(test_input)
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


def test_length_classattr():
    assert PublicKey.LENGTH == 32


def test_bytes_representation():
    data = (
        b"6\x8d-\x96\xcf\xe7\x93G~\xe0\x17r\\\x9c%\x9a\xab\xa6"
        b"\xa9\xede\x02\xbf\x83=\x10,P\xfbh\x8ev"
    )
    pubkey = PublicKey(data)
    assert bytes(pubkey) == data


def test_equality():
    assert PublicKey.new_default() == PublicKey.new_default()


def test_create_with_seed():
    """Test create with seed"""
    default_public_key = PublicKey.new_from_str("11111111111111111111111111111111")
    derived_key = PublicKey.create_with_seed(
        default_public_key, "limber chicken: 4/45", default_public_key
    )
    expected = PublicKey.new_from_str("9h1HyLCW5dZnBVap8C5egQ9Z6pHyjsh5MNy83iPqqRuq")
    assert derived_key == expected


def test_create_program_address():
    """Test create program address."""
    program_id = PublicKey.new_from_str("BPFLoader1111111111111111111111111111111111")
    program_address = PublicKey.create_program_address([b"", bytes([1])], program_id)
    assert program_address == PublicKey.new_from_str(
        "3gF2KMe9KiC6FNVBmfg9i267aMPvK37FewCip4eGBFcT"
    )

    program_address = PublicKey.create_program_address(
        [bytes("☉", "utf-8")], program_id
    )
    assert program_address == PublicKey.new_from_str(
        "7ytmC1nT1xY4RfxCV2ZgyA7UakC93do5ZdyhdF3EtPj7"
    )

    seeds = [bytes("Talking", "utf8"), bytes("Squirrels", "utf8")]
    program_address = PublicKey.create_program_address(seeds, program_id)
    assert program_address == PublicKey.new_from_str(
        "HwRVBufQ4haG5XSgpspwKtNd3PC9GM9m1196uJW36vds"
    )

    program_address = PublicKey.create_program_address(
        [bytes(PublicKey.new_from_str("SeedPubey1111111111111111111111111111111111"))],
        program_id,
    )
    assert program_address == PublicKey.new_from_str(
        "GUs5qLUfsEHkcMB9T38vjr18ypEhRuNWiePW2LoK4E3K"
    )

    program_address_2 = PublicKey.create_program_address(
        [bytes("Talking", "utf8")], program_id
    )
    assert program_address_2 != program_address

    # https://github.com/solana-labs/solana/issues/11950
    seeds = [
        bytes(PublicKey.new_from_str("H4snTKK9adiU15gP22ErfZYtro3aqR9BTMXiH3AwiUTQ")),
        bytes.fromhex("0200000000000000"),
    ]
    program_address = PublicKey.create_program_address(
        seeds, PublicKey.new_from_str("4ckmDgGdxQoPDLUkDT3vHgSAkzA3QRdNq5ywwY4sUSJn")
    )
    assert program_address == PublicKey.new_from_str(
        "12rqwuEgBYiGhBrDJStCiqEtzQpTTiZbh7teNVLuYcFA"
    )


def to_uint8_bytes(val: int) -> bytes:
    """Convert an integer to uint8."""
    return val.to_bytes(1, byteorder="little")


def test_find_program_address():
    """Test create associated_token_address."""
    program_id = PublicKey.new_from_str("BPFLoader1111111111111111111111111111111111")
    program_address, nonce = PublicKey.find_program_address([b""], program_id)
    assert program_address == PublicKey.create_program_address(
        [b"", to_uint8_bytes(nonce)], program_id
    )
