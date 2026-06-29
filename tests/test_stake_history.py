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


def test_stake_history_entry_constructors() -> None:
    a = StakeHistoryEntry.with_effective(5)
    assert (a.effective, a.activating, a.deactivating) == (5, 0, 0)
    b = StakeHistoryEntry.with_effective_and_activating(5, 6)
    assert (b.effective, b.activating, b.deactivating) == (5, 6, 0)
    # with_deactivating sets effective to the deactivating value too (matches upstream)
    c = StakeHistoryEntry.with_deactivating(7)
    assert (c.effective, c.activating, c.deactivating) == (7, 0, 7)
