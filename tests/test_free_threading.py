import os
import subprocess
import sys
import sysconfig
from concurrent.futures import ThreadPoolExecutor
from json import dumps, loads
from threading import Barrier, Lock
from unittest import TestCase, skipUnless

from solders.hash import Hash
from solders.keypair import Keypair
from solders.message import Message
from solders.pubkey import Pubkey
from solders.rpc.responses import GetBlockResp
from solders.signature import Signature
from solders.transaction import VersionedTransaction

IS_FREE_THREADED = sysconfig.get_config_var("Py_GIL_DISABLED") == 1


@skipUnless(IS_FREE_THREADED, "requires a free-threaded CPython build")
class TestFreeThreading(TestCase):
    """Exercise solders while CPython's GIL is disabled."""

    def test_import_keeps_gil_disabled(self) -> None:
        """Importing solders must not enable the GIL."""
        self.assertFalse(sys._is_gil_enabled())  # type: ignore[attr-defined]
        # PYTHON_GIL=0 would hide an extension silently enabling the GIL.
        result = subprocess.run(
            [
                sys.executable,
                "-c",
                "import sys; assert not sys._is_gil_enabled(); "
                "import solders.solders; assert not sys._is_gil_enabled()",
            ],
            env={key: value for key, value in os.environ.items() if key != "PYTHON_GIL"},
            capture_output=True,
            text=True,
            timeout=30,
            check=False,
        )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

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

    def test_shared_read_only_values(self) -> None:
        """Read and sign with the same native objects from several threads."""
        workers = 8
        barrier = Barrier(workers, timeout=30)
        keypair = Keypair.from_seed(bytes([7]) * 32)
        pubkey = keypair.pubkey()
        message = b"shared free-threaded values"
        signature = keypair.sign_message(message)
        digest = Hash.hash(bytes(signature))
        transaction = VersionedTransaction(Message([], pubkey), [keypair])
        expected_transaction = bytes(transaction)

        def work() -> None:
            barrier.wait()
            for _ in range(128):
                self.assertEqual(keypair.sign_message(message), signature)
                self.assertTrue(signature.verify(pubkey, message))
                self.assertEqual(Hash.hash(bytes(signature)), digest)
                self.assertEqual(bytes(transaction), expected_transaction)
                self.assertEqual(
                    VersionedTransaction.from_bytes(bytes(transaction)), transaction
                )

        with ThreadPoolExecutor(max_workers=workers) as pool:
            futures = [pool.submit(work) for _ in range(workers)]
            for future in futures:
                future.result(timeout=60)

    def test_concurrent_rpc_normalization(self) -> None:
        """Parse independent responses while preserving an opaque parsed payload."""
        payload = {"programIdIndex": 0, "accounts": []}
        response = {
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "blockhash": str(Hash.default()),
                "previousBlockhash": str(Hash.default()),
                "parentSlot": 0,
                "transactions": [
                    {
                        "transaction": {
                            "signatures": [],
                            "message": {
                                "accountKeys": [],
                                "recentBlockhash": str(Hash.default()),
                                "instructions": [
                                    {
                                        "programIdIndex": 0,
                                        "accounts": [],
                                    },
                                    {
                                        "program": "custom",
                                        "programId": str(Pubkey.default()),
                                        "parsed": payload,
                                    },
                                ],
                            },
                        },
                        "meta": None,
                    }
                ],
            },
        }
        raw = dumps(response)
        expected_response = GetBlockResp.from_json(raw)
        assert isinstance(expected_response, GetBlockResp)
        expected = expected_response.to_json()
        instructions = loads(expected)["result"]["transactions"][0]["transaction"][
            "message"
        ]["instructions"]
        self.assertEqual(instructions[0]["data"], "")
        self.assertEqual(instructions[1]["parsed"], payload)
        workers = 8
        barrier = Barrier(workers, timeout=30)

        def work() -> None:
            barrier.wait()
            for _ in range(64):
                parsed = GetBlockResp.from_json(raw)
                assert isinstance(parsed, GetBlockResp)
                self.assertEqual(parsed.to_json(), expected)

        with ThreadPoolExecutor(max_workers=workers) as pool:
            futures = [pool.submit(work) for _ in range(workers)]
            for future in futures:
                future.result(timeout=60)

    def test_shared_transaction_mutation_is_checked(self) -> None:
        """Accept complete snapshots or PyO3 borrow errors during conflicting access."""
        workers = 8
        barrier = Barrier(workers, timeout=30)
        first = Signature.default()
        second = Keypair.from_seed(bytes([9]) * 32).sign_message(b"replacement")
        message = Message.default()
        transaction = VersionedTransaction.populate(message, [first])
        valid_snapshots = {
            bytes(VersionedTransaction.populate(message, [first])),
            bytes(VersionedTransaction.populate(message, [second])),
        }

        def work(worker: int) -> None:
            barrier.wait()
            for _ in range(256):
                try:
                    if worker % 2:
                        transaction.signatures = [second if worker % 4 == 1 else first]
                    else:
                        snapshot = bytes(transaction)
                        self.assertIn(snapshot, valid_snapshots)
                except RuntimeError as exc:
                    self.assertIn(
                        str(exc).lower(), ("already borrowed", "already mutably borrowed")
                    )

        with ThreadPoolExecutor(max_workers=workers) as pool:
            futures = [pool.submit(work, worker) for worker in range(workers)]
            for future in futures:
                future.result(timeout=60)
        self.assertIn(bytes(transaction), valid_snapshots)
        # Scheduling need not produce a conflict; do not require a borrow error.

    def test_shared_transaction_with_caller_lock(self) -> None:
        """Use a caller lock to make each mutation and read a single operation."""
        workers = 8
        barrier = Barrier(workers, timeout=30)
        lock = Lock()
        transaction = VersionedTransaction.default()
        signatures = [
            Keypair.from_seed(bytes([seed]) * 32).sign_message(b"locked")
            for seed in range(workers)
        ]

        def work(worker: int) -> None:
            barrier.wait()
            for _ in range(64):
                with lock:
                    transaction.signatures = [signatures[worker]]
                    self.assertEqual(transaction.signatures, [signatures[worker]])

        with ThreadPoolExecutor(max_workers=workers) as pool:
            futures = [pool.submit(work, worker) for worker in range(workers)]
            for future in futures:
                future.result(timeout=60)
