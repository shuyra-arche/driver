# Driver Cluster

Aggregate for the Driver Cluster. The only migrated runtime Unit in this slice
is Hub. Other Driver Unit runtimes remain pending; this repository does not claim
a complete Driver Cluster or Source integration.

## Members

| Path | Kind | Identity / role |
| --- | --- | --- |
| `support` | local pure package | `driver-support` transport helper kept for aggregate-local work. Runtime Unit closures and Source consumers use immutable public `driver-support` `8e6de5a77aa52184bb6f7fbbcbc961f779e94629` with System Face5a. |
| `cluster` | local pure package | Explicit aggregate declaration; no runtime dependency on Hub server or Peer types. |
| `units/hub` | fixed public submodule | Hub Unit `daa23b2611fee0c503bc6bd31a16121d97698ad6`, extending canonical Hub API `cccef3f325198c8390007ae88b064bae0f21f93e`. |
| `units/router` | fixed public submodule | Router Unit `aa3033a7bd25b5d4199a50a6c00538475b62ef4c` (API + client), containing canonical Router API `94fec223f3c5df6c99a49527a65d016d88de3af4`; server/declaration/shared support/runtime remain Source/pending. |

`units/hub` owns its API/client/server/declaration and 65 runtime + 19 canonical
host tests. Its public proof passed 84 host cases and RV debug/release Hub ELFs.
Those tests are inside the Unit, not mirrored here. The aggregate does not mount
Unit paths as workspace members.

`units/router` now publishes the API plus the original client. Router server,
declaration, shared Context/Device/Fail support and runtime remain Source; no fake
Router UnitFile or Router runtime is declared. The Unit's public leaf proof
passed; aggregate public proof and Source/Kernel/System/QEMU acceptance remain
pending (see `VALIDATION.md` and `ACCEPTANCE.md`).

## Declarations

`driver-cluster` reexports only the public Hub declaration as `hub`. `UNITS`
contains the current migrated Hub Unit only. `IMAGE_BUILDS` contains one Hub
build and delegates every leaf BUILD field except the manifest prefix:
`units/hub/server/Cargo.toml`. The Unit BUILD itself is `server/Cargo.toml`;
future Source intake uses `components/driver/units/hub/server/Cargo.toml`.
Package/bin/image `hub-server`/`hub`/`hub`, empty features, RV64 release, and
default features remain unchanged.

## Pending

UART, RTC, PCI, IOMMU, virtio-blk and the device supervisor Unit runtimes remain
pending. Actual Source `Kernel3aa`/`System9001` SMP4/8 DMA, authorization,
claim/grant, fence, close/inspect/settle, reclamation and generation acceptance
remains pending and mandatory; host tests are not a substitute. Source Phase4 is
not assigned. Root owns selected version records, provenance, Git/submodule
metadata, locks and public proof. See `ACCEPTANCE.md`.
