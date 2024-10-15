from solders.pubkey import Pubkey
from solders.token.associated import get_associated_token_address


def test_ata() -> None:
    wallet_address = Pubkey.from_string("5d21Nx19eZBThbExCn1ESAk3RGmE8Rdp9PKMWZ2VedSK")
    token_mint = Pubkey.from_string("3CqfBkrmRsK3uXZaxktvTeeBkJp4yeFKs4mUi2jhKExz")
    assert get_associated_token_address(
        wallet_address, token_mint
    ) == Pubkey.from_string("Aumq2SPVzZccYL3UAhvXoDDkNYLZr2zpyxLuJiyx79te")
    token22_id = Pubkey.from_string("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb")
    assert get_associated_token_address(
        wallet_address, token_mint, token22_id
    ) == Pubkey.from_string("4xoV4cxTM3GcaWP7bKbUdu2Gp9P9nEgpmCPV8ykFGo4U")
