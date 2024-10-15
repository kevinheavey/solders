from typing import Optional

from solders.pubkey import Pubkey

def get_associated_token_address(
    wallet_address: Pubkey,
    token_mint_address: Pubkey,
    token_program_id: Optional[Pubkey] = None,
) -> Pubkey: ...
