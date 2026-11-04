# Driver aggregate acceptance

Status: **PENDING**. No Source Kernel3aa/System9001 integration run, QEMU
acceptance, aggregate native proof, or public aggregate proof has been performed
or claimed by the worker. The Router Unit `aa3033a…` leaf proof passed
independently; it does not establish this aggregate's own proof.

## Selected versions

- Hub Unit: `daa23b2611fee0c503bc6bd31a16121d97698ad6` (public proof passed:
  65 runtime + 19 canonical host cases; RV debug/release Hub ELFs).
- Canonical Hub API: `cccef3f325198c8390007ae88b064bae0f21f93e`.
- Router Unit: `aa3033a7bd25b5d4199a50a6c00538475b62ef4c` (API + client,
  independent public leaf proof passed). Canonical Router API remains
  `94fec223f3c5df6c99a49527a65d016d88de3af4`; Router
  server/declaration/runtime remain Source/pending. Context/Device/Fail now have
  a separate `driver-shared` provider; Source intake remains pending.
- Canonical `driver-support`: `8e6de5a77aa52184bb6f7fbbcbc961f779e94629`,
  preserving System Face `5a792ff2d39901a970925b1a6d63206adf6c16d8`.
- Kernel `3aa0a155f825fbf377fc8d04009472ca67cbbf8c`; System runtime
  `9001af65f97501dfa490c9397c61d593c871136d`.

## Required Source integration gates — all PENDING

| Gate | Required proof | Status |
| --- | --- | --- |
| Combined capture, SMP4 | Actual Source composition at the selected aggregate, real QEMU, raw exit and serial evidence | PENDING |
| Combined capture, SMP8 | Same at SMP8, including concurrency and generation ordering | PENDING |
| Owner/authentication | Real owner/authorization and foreign/self/third-party rejection paths | PENDING |
| Claim/grant/vacate | Real claim, grant delivery, failed delivery, owner exit and next-claim behavior | PENDING |
| DMA binding | Prepare/close/inspect/settle ordering and fence preservation | PENDING |
| Reclamation | Final detach/reclaim, no leaked binding, no premature rebind | PENDING |
| Generations/checkpoints | Monotonic generations and checkpoint/admission stability | PENDING |
| Selected version records | Actual aggregate full SHA plus fixed Unit/API/support revisions | PENDING |
| Public recursive proof | Clean public checkout, exact gitlinks, locked metadata | PENDING |

Root first publishes the independently validated candidate aggregate, then pins
it for a trial Source intake and runs the real SMP4 and SMP8 checkpoint gates.
Failure at either blocks accepting the tested version combination and delivering
the Source PR. No host mock, mirrored test, or constant assertion substitutes for
these gates. No new mirror tests were added.

Root independently selects and executes native checks and owns generated locks,
provenance, Git/submodule records, metadata, and public proof.

## `driver-shared` provider gates — local PASS, publication pending

The provider candidate in this repository's `shared/` passed Root's local gates
under a shared build lock. These are provider-local results; they do not prove a
public checkout, Source intake or any kernel/runtime behaviour.

| Gate | Result |
| --- | --- |
| Gate commands | 12/12 raw exit 0: item/body equality, both metadata pairs online and offline for the root and `tests/shared`, host shared tests, host workspace check, host debug/release, RV workspace check, RV debug/release |
| Item comparison | All 12 original shared items AST/body token-identical to Source `c975cec`, ignoring docs/imports/module layout |
| Metadata identity | The eleven selected external packages resolve to their exact Git revisions; the path packages are workspace members (`driver-shared`, `driver-support`, `driver-cluster`), and `tests/shared` adds only the new `driver-shared` path |
| Host tests | Four pure `Fail` → `Exit`/`Report::parts` cases pass (nonempty note, empty note → `None`, `Result::Err` code/text fidelity, `Result::Ok` = `EXIT_OK`/no note); signature and field identity probes are compile-time only and are not runtime cases |
| Builds | Host and `riscv64gc-unknown-none-elf` workspace checks, plus `driver-shared` library builds in debug and release |

`driver-support` remains a local path workspace package with `Peer` bytes
matching `8e6de5a…`; that is a byte comparison, not a claim of Git workspace
identity. Underlying `support/` bytes, `cluster/` and the Unit gitlinks are
unchanged, and the pre-existing lock tuples are preserved.

| Gate | Status |
| --- | --- |
| Full public clean checkout of the candidate revision | REQUIRED BEFORE CONSUMER — not yet performed or claimed |
| Source intake (`programs` dependency and `shared/mod.rs` reexport) | OUT OF SCOPE for this provider-only slice |
| New Source binaries, QEMU/SMP runs, Router/RTC/server runtime migration | OUT OF SCOPE / PENDING; not claimed |

The 19 Source host tests cited earlier in this file are baseline revalidation
only, not candidate runtime proof. No whole-Driver or full-health claim is made.
