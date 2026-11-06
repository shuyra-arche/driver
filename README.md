# Driver Cluster

Driver is Shuyra's build-time collection of device Units and shared driver support.
[Vanllia](https://github.com/shuyra-arche/vanllia) selects Units and assembles the runtime deployment.

## Layout

- `units/`: fixed Git submodules for Hub, Router, PCI, IOMMU, UART, RTC and VirtIO block. Each provider owns its `unit.toml`, API and server.
- `shared/`: Context, Device and failure adapters.
- `support/`: the Peer transport helper.
- `cluster/`: compatibility Rust declaration registry for Hub, Router, UART and RTC only.
- `tests/cluster/` and `tests/shared/`: independent host test workspaces.

TOML discovery includes all seven providers; the compatibility registry does not.
The gitlinks and `.gitmodules` are the source of truth for selected revisions.
A Cluster is a build organization, not a runtime Unit.

## Build and test

Install Rust through rustup; `rust-toolchain.toml` selects nightly-2026-09-04
and the RV64 target. From this checkout:

```sh
git submodule update --init --recursive
cargo test --locked --manifest-path tests/cluster/Cargo.toml --target x86_64-unknown-linux-gnu
cargo test --locked --manifest-path tests/shared/Cargo.toml --target x86_64-unknown-linux-gnu
cargo build --locked -p driver-cluster --lib --target riscv64gc-unknown-none-elf --release
```

Network access is needed for uncached public dependencies. Product image builds and
runtime acceptance use Vanllia. Host tests do not establish device, DMA or SMP behavior.
Standalone dependency selections in `Cargo.toml` use `[replace]`; each consumer
owns its combined source selection.

See [source attribution](PROVENANCE.md). Contribution and PR rules are maintained
in the [organization guide](https://github.com/shuyra-arche/vanllia/blob/main/CONTRIBUTING.md).
