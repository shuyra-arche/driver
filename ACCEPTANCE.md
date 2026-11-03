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
  server/declaration/shared support/runtime remain Source/pending.
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
