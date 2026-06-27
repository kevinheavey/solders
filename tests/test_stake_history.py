from solders.stake_history import StakeHistoryEntry


def test_stake_history_entry_setters() -> None:
    """Regression: activating/deactivating setters must not write to effective."""
    entry = StakeHistoryEntry(effective=1, activating=2, deactivating=3)
    entry.effective = 10
    entry.activating = 20
    entry.deactivating = 30
    assert entry.effective == 10
    assert entry.activating == 20
    assert entry.deactivating == 30
