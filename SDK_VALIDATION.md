# Canonical SDK dependency validation

The initial SDK validation below used the published Driver checkpoint
`bb3427e179e87b640e0d51115a44544fae231252`. At that stage only dependency
manifests, the two affected locks and documentation changed. Context, Device,
Fail, Peer, Cluster, Unit and test implementations, all four Unit gitlinks,
and the historical RTC/UART validation records were unchanged.

## Default branch integration

The candidate also incorporates the actual default branch
`codex/driver-cluster` at `abe8d1a70cfce04be50fa7ebbe5527830bc86ca0`, preserving
all seven provider gitlinks: Hub, Router, UART, RTC, IOMMU, PCI and VirtIO block.
Relative to that default, all Rust and Unit implementations, `.gitmodules` and
the seven gitlinks are identical; the SDK change still consists of manifests,
locks and documentation.

The automatic merge tree `44f26fe033c68023ffa389c7e4231ae23083464e`, before this
documentation clarification, independently passed the same raw locked/offline
RV64 workspace check and release build below. Its active normal/build graph
retains one canonical API owner, and the two lock hashes below are unchanged.
The four-plus-two host fixture results remain evidence from the initial
four-provider SDK checkpoint, not new seven-Unit runtime acceptance.

## Public source selection

| Package | Selected public revision |
| --- | --- |
| Execution | `ea49192e5df8bf744e083ced23ff0908b5bab3a3` |
| System client/support | `8eea7c9039cac0e6ada4a59c612dc6c98ef9bc45` |
| Env | `f54f9a1ab3f0df3631fb80ff2bf5d0cfaecbe927` |
| Wire | `4b7619fda4bbd202e9a5c952872a84c2145c681e` |
| Mold | `1addd302789e6a1c0dea6a7adf27e975a05d5ada` |
| Kernel API and Client | `90d43dbf3ee43fd146f487718eb39e88f2507c05` |

The direct Execution pins in `shared` and `tests/shared` select the same
Client-backed adapter. Each workspace root selects the three forwarding SDK
packages and System client/support with five exact Cargo Package ID
`[replace]` entries. This also covers Hub client's indirect old System client
edge, which otherwise brings back the old Execution and its native `links`
identity. Execution's `links = "execution"` and linker metadata are preserved.
No API, transport or lifecycle implementation is copied into Driver.

Cargo marks `[replace]` deprecated. These entries are a bounded compatibility
choice for the existing immutable dependency graph, not a second SDK. Their
keys match a specific old URL, revision, package and version. A changed old pin
requires renewed resolution and graph checks; remove the entries when those
dependency edges select the canonical packages themselves. Replacement records
in a lock are not additional active type owners.

Root replacements do not propagate when Driver is a dependency. The consuming
workspace, including IOMMU or the Vanllia CLI, owns its source selection. The
raw commands below need no external config, path override, URL alias or source
copy. Both generated locks contain public Git sources, and their registry
package versions and checksums match the baseline exactly.

## Validation

Validation used installed `nightly-2026-10-09`; historical toolchain files and
the earlier `09-04` results were not relabelled as this run. The baseline raw
RV64 check and both existing host fixtures passed before the dependency change.
After selection, these raw commands passed:

```sh
cargo check --locked --offline --workspace --target riscv64gc-unknown-none-elf
cargo build --locked --offline --workspace --target riscv64gc-unknown-none-elf --release
cargo test --locked --offline --manifest-path tests/shared/Cargo.toml --target x86_64-unknown-linux-gnu
```

The workspace builds three libraries, not an ELF entry. Shared's four original
host assertions passed unchanged. The unchanged `tests/cluster` fixture passed
its two original assertions after initializing the four fixed Unit submodules.
It retains its own earlier SDK pins and lock; it is not a canonical SDK fixture.

Target `normal,build` trees were checked for `driver-shared`, `driver-support`
and `driver-cluster` on RV64 and `driver-shared-host-tests` on the host. Each has
exactly one Kernel API/Client owner at the revision above. Shared and its host
fixture have one Execution and two selected System packages; Support has one
Execution and one System client. Cluster has no Execution or System client.
No active tree contains the old Execution or old System client/support.

Lock SHA-256 values:

- `Cargo.lock`: `9b5b3c7cb6a5c797ff2edef2aa1b7f342d79092a4de35be16583d343c739f090`
- `tests/shared/Cargo.lock`: `760893eac800ff12e2af45b5dd4ca308d8dba95deb32005fcb11fd4d5cc0e3d5`

The four Unit source pins and `tests/cluster/Cargo.lock` remain unchanged.
No Unit server ELF, QEMU, hardware I/O or new Runtime exit policy is established
by these library and interface checks; the product composition validates its
actual images separately.
