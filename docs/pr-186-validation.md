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
  execution environment failed to connect. The updated source was instead built
  and tested in the fork, as documented below.

CI commands:
- Existing Build workflow: build abi3 wheels, test on the configured regular
  CPython interpreter, and run
  `python -m unittest tests.test_free_threading` under CPython 3.14t with
  `PYTHON_GIL=0` on the configured free-threaded release targets.
- Lint: `uv run make lint` (Clippy with/without default features, Ruff, mypy).
- Rust normalizer unit tests: `uv run cargo test -p solders-rpc-responses --lib`.

## Executed CI results

The reviewed source/test commit is
[807ab2f](https://github.com/RustedBytes/solders/commit/807ab2f542a037f406b94b1d1e751d45625c8b9c).
Validation executed at
[f3ec459](https://github.com/RustedBytes/solders/commit/f3ec4592b62c1be95f4fffb1a61b0e6c00a9e21a)
in the fork's `pr-186-validation` branch. Git tree comparisons verified identical
source and test blobs; only the two workflow push branch filters differ.
The validation-only workflow changes were not added to the upstream PR branch.

- [Build 38406163686](https://github.com/RustedBytes/solders/actions/runs/38406163686):
  completed successfully on 2026-10-13, all eight jobs.
- [Lint 38406164028](https://github.com/RustedBytes/solders/actions/runs/38406164028):
  completed successfully on 2026-10-13.
- `uv run make lint`: Clippy with and without default features, Ruff and mypy
  passed. Mypy reported no issues in 73 source files.
- `uv run cargo test -p solders-rpc-responses --lib`: 3 tests passed.

| Build job | Regular CPython tests | CPython 3.14t tests |
| --- | --- | --- |
| macos_x86_64 | 334 passed, 6 skipped | Covered by universal2 job |
| macos_universal2 | 334 passed, 6 skipped | 6 passed |
| windows | 334 passed, 6 skipped | 6 passed |
| linux-x86_64 | 334 passed, 6 skipped | 6 passed |
| linux-aarch64 | 334 passed, 6 skipped | 6 passed |
| musllinux-x86_64 | 334 passed, 6 skipped | 6 passed |
| musllinux-aarch64 | 334 passed, 6 skipped | 6 passed |
| sdist | Source archive built | Not applicable |

The six skips in regular CPython are the free-threading tests' expected
interpreter guard. The six tests in each free-threaded job ran successfully,
including the fresh import subprocess with `PYTHON_GIL` removed. macOS
universal2 was built for both architectures but its Python tests ran on the
workflow's Intel runner; this does not establish runtime testing on macOS ARM64.

Observed runtime/tool versions: Rust 1.95.0 (repository toolchain pin), PyO3
0.29.2, maturin 1.14.1, Ruff 0.1.6 and mypy 0.991. Regular test runtimes were
CPython 3.10.22 on Linux/glibc, 3.10.11 on Windows, 3.10.21 on macOS and
3.14.8 on Alpine/musl. The separate free-threaded wheels used CPython 3.14t.
Intermediate interpreters 3.11–3.13 were not separately exercised in this review.

The first fork lint attempt identified four mypy errors in the new tests:
RPC error/result unions and compiled/parsed instruction unions needed explicit
type narrowing. These were corrected before the successful validation above;
the production Rust fix did not change during that correction.

Upstream runs for the source/test commit did not execute any jobs and have
`action_required`:
[Build 38406167272](https://github.com/kevinheavey/solders/actions/runs/38406167272)
and [Lint 38406166883](https://github.com/kevinheavey/solders/actions/runs/38406166883).
They are not passing upstream checks. The successful fork runs provide the
executed verification; upstream workflow approval remains a maintainer action.
