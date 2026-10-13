# PR #186: RPC normalization and free-threading review

Review baseline: `58a897d35b27c00ac631e317b7473487af62f391` on
`RustedBytes/solders:add-no-gil`. Review date: 2026-10-13.

## Confirmed normalization defect

The original recursive normalizer visits every JSON object in a block response.
A valid parsed instruction can carry an arbitrary `parsed` payload such as
`{"programIdIndex": 0, "accounts": []}`. The original implementation inserts
`"data": ""` into that payload and into matching nested objects. This changes
application data unrelated to compiled instructions.

Normalization now visits only:
- `result.transactions[].transaction.message.instructions[]`;
- `result.transactions[].meta.innerInstructions[].instructions[]`.

Only compiled instruction objects with an unsigned numeric `programIdIndex`,
array `accounts`, and no `programId` or `parsed` are candidates. Only an absent
`data` is defaulted. Existing empty/populated strings, nulls and wrong types are
left for the existing deserializer to handle. No dependency versions, interpreter
requirements, public signatures or wheel targets have been changed.

Python regressions cover both instruction locations and missing, empty and
populated data, opaque parsed payloads at both locations, invalid explicit data
and RPC errors. Whole typed response serialization is compared with the expected
response to detect changes to other instructions or metadata.
Rust unit tests additionally compare complete JSON values, covering opaque
extension/error data, non-compiled instructions, null/scalar/wrong-shape
containers, encoded transactions and idempotence. They run in the lint workflow.

## Free-threading assessment

The baseline tests cover import and operations on independent objects only.
The import test previously ran solely with `PYTHON_GIL=0`, which overrides an
extension's GIL requirement and can conceal an import enabling the GIL.

The tests now also check a fresh subprocess without that environment override;
shared read-only Keypair, Pubkey, Signature and VersionedTransaction values;
parallel block response parsing; conflicting signature setters/serialization
on a shared VersionedTransaction; and caller-locked mutation/read sequences.
Barriers coordinate worker start without sleeps. Conflicting access may either
complete or raise the documented PyO3 borrow RuntimeError; the stress test does
not require a scheduling-dependent conflict. Successful serialization must be
one complete allowed snapshot.

Source inspection covered the repository's 74 Rust source files outside test
fixtures. No first-party `unsafe`, `static mut`, `UnsafeCell`, `RefCell`,
`unsendable`, Rust locks or detach calls were found in that scan. Reviewed
signing methods use `&self`; mutable transaction signature setters and LiteSVM
operations use `&mut self`, and getters clone native data. PyO3 0.29's borrow
checking protects conflicting accesses to these pyclass values. No unsafe
shared-access defect was confirmed, so no object synchronization implementation
was changed. The README's caller-lock requirement remains applicable.

This is a focused source review plus stress coverage, not an exhaustive
dependency audit or a proof of race freedom. LiteSVM's complete concurrent
execution behavior is not established by the transaction tests.

References:
- https://pyo3.rs/v0.29.0/free-threading.html
- https://solana.com/docs/rpc/json-structures

## Checks actually performed

- Retrieved PR metadata/diff and source files through GitHub.
- Confirmed that baseline Build run 37836898280 and Lint run 37836898365 succeeded.
  Those results apply to the baseline, not these changes.
- Reproduced the opaque-payload mutation with a direct JavaScript translation
  of the original normalizer. This is algorithm evidence, not execution of the
  Rust extension.
- Reviewed all first-party Rust source files for the concurrency constructs
  listed above and inspected the relevant methods and generated method macros.
- Local build, lint and Python/Rust test execution were unavailable because the
  execution environment failed to connect. Updated-commit CI results must be
  recorded separately after execution.

CI commands:
- Existing Build workflow: build/test CPython 3.10 wheels and run
  `python -m unittest tests.test_free_threading` under CPython 3.14t with
  `PYTHON_GIL=0` on the configured free-threaded release targets.
- Lint: `uv run make lint` (Clippy with/without default features, Ruff, mypy).
- Rust normalizer unit tests: `uv run cargo test -p solders-rpc-responses --lib`.
