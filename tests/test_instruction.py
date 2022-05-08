from solders import Instruction, AccountMeta, Pubkey


def test_accounts_setter() -> None:
    ix = Instruction(
        Pubkey.default(), b"1", [AccountMeta(Pubkey.new_unique(), True, True)]
    )
    new_pubkey = Pubkey.new_unique()
    new_accounts = [AccountMeta(Pubkey.new_unique(), True, True)]
    ix.accounts = new_accounts
    assert ix.accounts == new_accounts
