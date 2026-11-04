# RTC/UART Aggregate Intake

This slice extends the Driver aggregate from Hub/Router to the four-Unit scope
Hub, Router, UART and RTC. The aggregate selects declarations and builds; the
Units keep ownership of declarations, startup dependencies and runtime.

## Immutable revisions

| Role | Revision |
| --- | --- |
| Driver base (PR3 URL fix) | `054dd7e6c3e2aa00e1f8f8d776f1fdebff731752` |
| Vanllia-selected Driver parent | `b82a20f36684c13808d89b0f1a5b4ce2796c2dbf` |
| RTC provider (`units/rtc`) | `f14cca51af02061350b2d1aa9f00f95c6dc06e15` |
| UART provider (`units/uart`) | `298bc8809f297f46bc7592aebc96540b7b492efc` |
| Existing Unit crate | `90396f0fda376e0693fb1b9553aa21864d241057` |
| Existing Hub provider | `daa23b2611fee0c503bc6bd31a16121d97698ad6` |
| Existing Router provider | `8a9416277539aefbc759c4f6519e1e8d93d63be3` |

The chain is Vanllia-selected `b82a20f...` -> Driver PR3 `054dd7e...` -> this
working branch (`codex/rtc-uart-intake`). This branch revision is published only
by Root after verification.

## Registration graph

`cluster/Cargo.toml` keeps the existing Hub and Router sources and adds:

```text
rtc-declaration = driver-rtc.git @ f14cca51af02061350b2d1aa9f00f95c6dc06e15
uart-declaration = driver-uart.git @ 298bc8809f297f46bc7592aebc96540b7b492efc
```

`cluster/src/lib.rs` reexports the real owner declarations and registers:

```text
UNITS:         hub, router, uart, rtc
IMAGE_BUILDS:  hub, router, uart, rtc
manifests:     units/hub/server/Cargo.toml
               units/router/server/Cargo.toml
               units/uart/server/Cargo.toml
               units/rtc/server/Cargo.toml
```

Every other `ImageBuild` field delegates to the owning Unit `BUILD`. The
aggregate adds no declaration copy, retry, rollback or synchronization state.

Startup dependencies remain Unit-owned:

```text
hub:    Operator, Identity
router: Operator, hub
uart:   Operator, hub, router
rtc:    Operator, hub, router
```

UART/RTC readiness, ending and namespaces are unchanged from their providers.

The resulting `driver-cluster` graph has one `rtc-api` source identity, resolved
from `rtc-declaration`'s Unit-internal `../api` path inside
`driver-rtc.git@f14cca51...`. The UART declaration has no UART API dependency.
Canonical `router-api@94fec223...` is shared by the Router and RTC declarations.
No `rtc-api`, `uart-api`, `uart-client`, `rtc-client` or server crate is added as
a separate cluster dependency.

## Pending Vanllia intake

Vanllia PR3 `1b619f996869c9d9f22e5cdededf4e127fd37554`
(`https://github.com/shuyra-arche/vanllia.git`) still points at Driver
`b82a20f...`, not at this aggregate. In repository-relative `composition/`:

- `composition/src/units/mod.rs` and `composition/src/units/uart.rs`/`rtc.rs`
  keep local UART/RTC declarations; `PROGRAMS` keeps product registration order.
- `composition/src/image_builds.rs` keeps `legacy_image!("uart")` and
  `legacy_image!("rtc")` entries (`legacy/sqware/programs/Cargo.toml`,
  `prog-uart`/`prog-rtc`).
- `composition/src/deployments.rs` keeps the product start order and the `rtc`
  supervised target.
- `composition/Cargo.toml` keeps `driver-cluster` at `b82a20f...` and a direct
  `rtc-api` from `driver-rtc.git@621be2b9946827be3de4245e29aa74ceb0f8cbd0`.

A later Vanllia intake must move `driver-cluster` to the published aggregate,
replace the local UART/RTC declarations with `driver_cluster::uart` and
`driver_cluster::rtc`, and replace the legacy image builds with delegated
UART/RTC `BUILD` fields except Vanllia-relative manifests. Program and
deployment order must be preserved. The unused direct `rtc-api` can be removed,
or every still-used RTC dependency (API/client/server/declaration) must be
aligned to `driver-rtc.git@f14cca51af02061350b2d1aa9f00f95c6dc06e15` so each
normal graph has exactly one RTC API source identity. Frozen sqware dev-only
fixtures may remain, but the sqware `rtc-api` must stay out of the normal graph.

## Pending Terminal intake

Terminal at `f07a0954f65472e9a16e878e52c5f1d34146bc64`
(`https://github.com/shuyra-arche/terminal.git`) uses `uart-api` and
`uart-client` from `uart.git@b690431496b0beaeb5fb7cafb42e60b4c8b38c9d`
(`server/Cargo.toml`). Terminal's own later PR must update both `uart-api` and
`uart-client` together to `driver-uart.git` at the selected full
`298bc8809f297f46bc7592aebc96540b7b492efc` SHA. Do not mix
`uart.git@b6904314...` with that source in one closure, and do not rename any
global Cargo source identity.

The frozen sqware dev-only fixtures are untouched. The standalone Driver
declaration graph can be coherent before Vanllia and Terminal coordinate their
intake. This PR makes no Vanllia, hardware, IRQ/MMIO, runtime, QEMU or
migration-completion claim.

## Verification

Root's candidate results are recorded in `RTC_UART_VALIDATION.md`: 31 commands
returned raw 0; six online/offline graph proofs passed; 19 shared/support/old-test
files and old lock package identities were preserved; the four recursive pins
were exact; each consumer API had one RTC/UART source; and four debug/release RV
ELF binaries were produced. On nightly-2026-09-04, 39 host test cases passed
(cluster 2 + shared 4 + RTC 11 + 10 + UART 6 + 6), including a final cluster
rerun, and host/RV workspace checks plus cluster debug/release library builds
passed. No QEMU or runtime behavior was exercised. Independent DeepSeek review
found no P0-P2 blocker; see `RTC_UART_VALIDATION.md`. Public fresh-checkout
verification is recorded separately at publication.
