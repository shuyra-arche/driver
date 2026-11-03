# Driver Cluster

Current approved first slice: **Hub only**. This tree is not a complete Driver
Cluster and does not claim the other Unit runtimes are migrated.

## Members

| Path | Package | Role |
| --- | --- | --- |
| `support` | `driver-support` | Shared pure transport/helper crate; one `Peer` implementation. |
| `units/router/api` | `router-api` | Pure Router API prerequisite only. Router runtime/client stay Source this slice. |
| `units/hub/client` | `hub-client` | Original Hub typed client, adapted only at crate boundaries. |
| `units/hub/server` | `hub-server` / bin `hub` | Original Hub server plus public `mold`/`execution` entry. |
| `units/hub/declaration` | `hub-declaration` | Hub `UNIT` and `BUILD` declaration. |
| `cluster` | `driver-cluster` | Explicit migrated-unit roster; reexports Hub declaration only. |
| `units/hub/tests/handoff` | owner tests | 21 Hub runtime handoff assertions. |
| `units/hub/tests/retention` | owner tests | 39 Hub retention/claim/protocol assertions. |
| `units/hub/tests/capability-import` | owner tests | 5 Hub claim/grant/vacate assertions. |
| `units/hub/tests/coordination` | retained API tests | 10 canonical Hub API coordination wire tests; not a replacement for runtime suites. |

## Identity and dependencies

- Canonical Hub API remains public
  `cccef3f325198c8390007ae88b064bae0f21f93e`; no active API definitions are
  copied into this repository.
- System Face identity remains public `system-client`
  `5a792ff2d39901a970925b1a6d63206adf6c16d8`; `driver-support` depends on that
  Git source, not a sibling System workspace.
- Original Hub and shared `Peer` bodies, names, field order, limits, DMA/fence
  ordering, authorization and reclamation remain preserved. Boundary changes
  are limited to `crate::core`, `system_support`, and `driver_support::peer`.
- Hub build: `units/hub/server/Cargo.toml`, package `hub-server`, bin `hub`,
  image `hub`, no features, release profile, default features enabled.

## Test ownership

The current Driver/Hub runtime suites are **65 Hub-owned Rust assertions**:
`handoff` 21, `retention` 39, `capability-import` 5. Every suite uses the actual
Hub production core/serve projections in this tree, with the original local
transport/resource fixtures. Exact source module, Unit file, and test-name
mapping is in `units/hub/tests/README.md`.

`units/hub/tests/coordination` contains the 10 canonical public Hub API
coordination tests and does not replace the 65 runtime assertions.

`programs/tests/device-recovery` (8 Rust + 126 Python) and the five mixed
`programs/tests/system-shape` selectors remain Source for this slice. The frozen
historical retention IOMMU selectors are intentionally not recreated.

## Pending

Router runtime/client, UART, RTC, PCI, IOMMU, virtio-blk and the device
supervisor remain pending. Router API is a pure prerequisite, not a claim that
the Router Unit runtime has moved. Source composition and Deployment selection
remain Root-owned. No native, RV, Kernel or System integration is claimed here.
