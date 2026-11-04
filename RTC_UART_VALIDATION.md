# RTC/UART aggregate validation

Candidate base: Driver PR #3 URL correction at
`054dd7e6c3e2aa00e1f8f8d776f1fdebff731752`, following Vanllia-selected
`b82a20f36684c13808d89b0f1a5b4ce2796c2dbf`.

Root ran these checks on 2026-11-04 using `nightly-2026-09-04`, explicit
`x86_64-unknown-linux-gnu` and `riscv64gc-unknown-none-elf` targets. Native
commands used the shared build lock and the 6 GiB free-space guard. All 31
candidate commands returned raw exit 0; the final aggregate test repeat also
returned 0.

| Executed check | Result |
| --- | --- |
| Real four-Unit registration/build routing tests | 2 passed |
| Existing Driver shared host tests | 4 passed |
| Existing RTC protocol / server-declaration host tests | 11 + 10 passed |
| Existing UART wire / declaration host tests | 6 + 6 passed |
| Locked online/offline metadata, six workspaces | Same package identities and resolved graphs |
| Host and RISC-V workspace library checks | Passed |
| Host and RISC-V `driver-cluster` libraries, debug/release | Passed |
| RTC and UART RISC-V binaries, debug/release | Four ELF64 RISC-V executables |
| Existing shared/support/old shared-test files | All 19 byte-identical |
| Existing lock package identities | Preserved; three new provider packages added |
| Exact recursive Unit pins and clean Unit worktrees | Passed |
| `git diff --check` | Passed |

The 39 host cases are distinct cases; the final aggregate repeat does not add
two more. Provider test executions here are fresh runs at the selected public
gitlinks, separate from their earlier provider PR receipts.

The Driver declaration graph contains one `rtc-api` at
`driver-rtc.git@f14cca51af02061350b2d1aa9f00f95c6dc06e15`, the corresponding
RTC declaration and `uart-declaration` at
`driver-uart.git@298bc8809f297f46bc7592aebc96540b7b492efc`. It contains no
runtime server or UART API. An independent external consumer metadata probe
selected API/client/server/declaration together for both Units and confirmed
one exact full-SHA Git source per Unit across all eight packages. Unit-local
relative API dependencies resolve within those Git sources. The audited normal
and build dependency closures contain no sqware; frozen dev fixtures were not
deleted.

All four new and retained Unit gitlinks were checked:

- Hub: `daa23b2611fee0c503bc6bd31a16121d97698ad6`.
- Router: `8a9416277539aefbc759c4f6519e1e8d93d63be3`.
- UART: `298bc8809f297f46bc7592aebc96540b7b492efc`.
- RTC: `f14cca51af02061350b2d1aa9f00f95c6dc06e15`.

All four binaries have ELF machine 243, executable type, a first LOAD at
`0x10000`, one `_start`, an entry within an executable LOAD segment, and no
interpreter. Root preserved compressed ELF bytes and SHA-256 records. The
validated registration/test code digest is
`ba2ed81d1bce135d978dbeb97e41c892aa36c2b6ae5846cde1bb0b0c59c00091`
(sorted records of path, NUL, file SHA-256 and LF for `cluster/Cargo.toml`,
`cluster/src/lib.rs` and `tests/cluster/src/lib.rs`).

Raw commands, outputs, dependency probe, graph/pin/preservation proof, binary
records and genuine DeepSeek group traces are retained under the task evidence
directory. A fresh genuine `opencode-go / deepseek-v4.1-flash` group owns this PR:
namer `session-ae494388-33da-442d-98eb-625bd72f858b`, worker
`session-a92bc388-fc02-46da-b543-2f92cf7ceeee`, and reviewer
`session-2849d05c-b2fc-407f-93ce-6994e900d7e1`. Persisted model events verify all
three selections. The worker authored registration/tests; Root ran native gates
and maintained generated locks, gitlinks and receipts. Independent static review
found no confirmed P0-P2 blocker. Its submodule-initialization documentation note
was addressed; manifest-content coverage remains a stated boundary of the
aggregate tests, supplemented by metadata and real provider builds.

Public fresh-checkout verification uses a separate target directory and is
recorded at publication in the PR and task receipt.

This validates Driver selection and standalone builds. Vanllia and Terminal
intake, startup, MMIO, interrupt behavior and QEMU remain unverified in this PR.
See [RTC_UART_INTAKE.md](RTC_UART_INTAKE.md) for the required consumer changes.
