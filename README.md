# Driver Cluster

The aggregate registers the independently published Hub and Router Units. Units
own their declarations and runtimes; the aggregate fixes their revisions and
orders their declarations and image builds. Source owns combined composition
and Deployment. Other Driver runtimes remain pending.

| Path | Role | Fixed public revision |
| --- | --- | --- |
| `units/hub` | Hub API, client, server and declaration | `daa23b2611fee0c503bc6bd31a16121d97698ad6` |
| `units/router` | Router API, client, server and declaration | `8a9416277539aefbc759c4f6519e1e8d93d63be3` |
| `cluster` | Pure `driver-cluster` declaration and image registry | This aggregate revision |
| `shared` | Existing Context, Device and Fail support | Bytes preserved from `e75b1e232f828666956fb7e02c61a7d84f13eb66` |
| `support` | Existing pure Peer transport helper | Bytes preserved from canonical `8e6de5a77aa52184bb6f7fbbcbc961f779e94629` |

`driver-cluster` reexports the real public declarations as `hub` and `router`.
`UNITS` and `IMAGE_BUILDS` contain Hub first, then Router. All build fields
delegate to the owning Unit except each aggregate-relative manifest prefix:
`units/hub/server/Cargo.toml` and `units/router/server/Cargo.toml`.
No Unit workspace is mounted in the aggregate workspace, no declaration is
copied, and the cluster has no runtime/server/Peer dependency.

Canonical Router API remains `94fec223f3c5df6c99a49527a65d016d88de3af4` and
Router client remains `aa3033a7bd25b5d4199a50a6c00538475b62ef4c`. Router server
uses immutable `driver-shared` at `e75b1e232f828666956fb7e02c61a7d84f13eb66`.
Hub API, support, System and all existing URL identities retain their pins.
Repository renames are not incorporated into Cargo sources in this slice.

Root's candidate checks passed: locked online/offline metadata for the workspace
and `tests/cluster`, two real aggregate registration/routing cases, host/RV
workspace checks and `driver-cluster` debug/release library builds. Frozen
support/shared/old tests, exact Unit gitlinks and old lock identities were checked.
Only the cluster's Router dependency and the new Router declaration lock entry
were added. See `VALIDATION.md` for scope and `ACCEPTANCE.md` for pending intake.

```sh
cargo test --locked --manifest-path tests/cluster/Cargo.toml --target x86_64-unknown-linux-gnu
cargo build --locked -p driver-cluster --lib --target riscv64gc-unknown-none-elf --release
```

Before Source consumes this aggregate, Root must verify a clean public full-SHA
checkout with exact recursive gitlinks and a separate build directory. Source
Router intake and real SMP4/8 acceptance then run in their own fresh role group
and serial queue. No QEMU, full Driver migration or complete historical scenario
matrix is claimed by these aggregate tests. Existing provider evidence is not
counted as a new execution of those tests.
