import sys
import sysconfig
from concurrent.futures import ThreadPoolExecutor
from unittest import TestCase, skipUnless

from solders.hash import Hash
from solders.keypair import Keypair
from solders.pubkey import Pubkey

IS_FREE_THREADED = sysconfig.get_config_var("Py_GIL_DISABLED") == 1


@skipUnless(IS_FREE_THREADED, "requires a free-threaded CPython build")
class TestFreeThreading(TestCase):
    """Exercise solders while CPython's GIL is disabled."""

    def test_import_keeps_gil_disabled(self) -> None:
        """Importing solders must not enable the GIL."""
        self.assertFalse(sys._is_gil_enabled())  # type: ignore[attr-defined]

    def test_concurrent_operations(self) -> None:
        """Independent solders values can be used concurrently."""

        def work(seed: int) -> tuple[str, str]:
            keypair = Keypair.from_seed(bytes([seed % 256]) * 32)
            signature = keypair.sign_message(b"free-threaded smoke test")
            pubkey = Pubkey.from_bytes(bytes(keypair.pubkey()))
            digest = Hash.hash(bytes(signature))
            return str(pubkey), str(digest)

        with ThreadPoolExecutor(max_workers=16) as pool:
            results = list(pool.map(work, range(512)))

        self.assertEqual(len(results), 512)
        self.assertEqual(len(set(results)), 256)
