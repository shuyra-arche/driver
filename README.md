# Driver Cluster

Current provider gitlinks and device-driver intake are recorded in
[DEVICE_DRIVER_INTAKE.md](DEVICE_DRIVER_INTAKE.md), continuing
[UNIT_TOML_INTAKE.md](UNIT_TOML_INTAKE.md). The revision table and Rust registry
below describe the retained legacy Cargo interface; TOML discovery reads the
provider-owned `unit.toml` files at the current gitlinks.

Recursive discovery now includes PCI, IOMMU and VirtIO block alongside Hub,
Router, UART and RTC. Their provider declarations own startup dependencies,
authentication and dynamic supplies; the aggregate only fixes their revisions.

The aggregate registers the independently published Hub, Router, UART and RTC
Units. Units own their declarations and runtimes; the aggregate fixes their
revisions and orders their declarations and image builds. Source owns combined
composition and Deployment.

| Path | Role | Fixed public revision |
| --- | --- | --- |
| `units/hub` | Hub API, client, server and declaration | `daa23b2611fee0c503bc6bd31a16121d97698ad6` |
| `units/router` | Router API, client, server and declaration | `8a9416277539aefbc759c4f6519e1e8d93d63be3` |
| `units/uart` | UART API, client, server and declaration | `298bc8809f297f46bc7592aebc96540b7b492efc` |
| `units/rtc` | RTC API, client, server and declaration | `f14cca51af02061350b2d1aa9f00f95c6dc06e15` |
| `cluster` | Pure `driver-cluster` declaration and image registry | This aggregate revision |
| `shared` | Existing Context, Device and Fail support | Implementation preserved from `e75b1e232f828666956fb7e02c61a7d84f13eb66`; SDK selection below |
| `support` | Existing pure Peer transport helper | Bytes preserved from canonical `8e6de5a77aa52184bb6f7fbbcbc961f779e94629` |

`driver-cluster` reexports the real public declarations as `hub`, `router`,
`uart` and `rtc`. `UNITS` and `IMAGE_BUILDS` contain Hub, Router, UART and RTC in
that order. All build fields delegate to the owning Unit except each
aggregate-relative manifest prefix: `units/<unit>/server/Cargo.toml`. No Unit
workspace is mounted in the aggregate workspace, no declaration is copied, and
the cluster has no runtime/server/Peer dependency.

Startup dependencies stay owned by the Units. UART and RTC declare Operator,
Hub and Router in that order; the existing Hub and Router declarations are
unchanged. The aggregate creates no retry, rollback or synchronization state.

Canonical Router API remains `94fec223f3c5df6c99a49527a65d016d88de3af4` and
Router client remains `aa3033a7bd25b5d4199a50a6c00538475b62ef4c`. Router server
uses immutable `driver-shared` at `e75b1e232f828666956fb7e02c61a7d84f13eb66`.
Unit manifests retain their existing provider pins and source URL identities.
Repository renames are not incorporated into existing Cargo source URLs.

The aggregate and shared host fixture now select the canonical Kernel SDK and
the matching System client/support packages from fixed public commits. Shared
uses Execution's Client-backed implementation. No Context, Device, Fail, Peer,
Unit or test body changes. See [SDK validation](SDK_VALIDATION.md) for the exact
source selection, raw library builds and current test results.

The earlier RTC/UART intake results are recorded in `RTC_UART_VALIDATION.md`:
31 commands returned
raw 0; six online/offline graph proofs passed; 19 shared/support/old-test files
and old lock package identities were preserved; all four recursive pins matched;
each consumer API had one RTC/UART source; and four debug/release RV ELF binaries
were produced. Host tests cover 39 distinct cases (cluster 2, shared 4, RTC 11 +
10, UART 6 + 6), including a final cluster rerun, under nightly-2026-09-04.
Host/RV workspace checks and cluster libraries passed in debug and release.
Public fresh-checkout verification and independent review receipts remain
pending; no QEMU or runtime result is claimed. See `RTC_UART_INTAKE.md` for
immutable revisions, the registration graph and Vanllia/Terminal prerequisites.

```sh
git submodule update --init --recursive
cargo test --locked --manifest-path tests/cluster/Cargo.toml --target x86_64-unknown-linux-gnu
cargo build --locked -p driver-cluster --lib --target riscv64gc-unknown-none-elf --release
```

No QEMU, hardware, full Driver migration or complete historical scenario matrix
is claimed by these aggregate tests.
