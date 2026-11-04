# Driver aggregate validation scope

The current aggregate has fixed Hub `daa23b2…` and Router `aa3033a…` Unit
submodules. Hub and Router carry separate public leaf proofs: Hub 84 host cases
and actual RV executables; Router Unit `aa3033a` passed nine byte-equality checks,
canonical API identity, online/offline metadata and host/RV debug/release
library builds, with API + client only and no runtime claim. That Router leaf
proof is not a proof of this aggregate. The aggregate still requires its own
locked metadata, pure host/RV libraries, exact recursive public gitlinks and a
clean intended tree before Source integration; current aggregate public proof is
PENDING. Real Source/Kernel3aa/System9001 SMP4/8 DMA/close/inspect/settle/reclaim
acceptance remains PENDING. No prior monorepo-layout pass is carried to changed
pins.

The following is retained history for the original Driver8e6 snapshot; it is not
a statement that the new aggregate or its Source runtime has already passed.

---

# First Hub slice validation

Root independently validated this partial Driver Cluster snapshot against Source
`293c15fc97a9d9a4295810d10fe804e4cdec63c4`, with nightly-2026-09-04.
These are provider-local results; public clean-checkout and Source integration
are separate recorded gates. The Driver roster is not fully migrated.

- Online metadata and actual `--locked --offline` metadata pass for the root
  workspace and four standalone host-test workspaces. All external Git sources
  use public complete revisions; production projections stay in this repository.
- Host tests on explicit `x86_64-unknown-linux-gnu`: handoff21, retention39,
  capability-import5, coordination10, all pass (75 total). The first65 are
  relocated Hub runtime assertions; coordination10 remains canonical API coverage.
- Production `cargo check --workspace --lib --target x86_64-unknown-linux-gnu
  --locked --offline` passes. This is a host-library check, not a claim that a
  freestanding RISC-V binary supports the host target.
- Actual `cargo build --workspace --target riscv64gc-unknown-none-elf --locked
  --offline --message-format=json` passes in debug and release. Cargo reports the
  `hub-server` package and `hub` binary. Both outputs are ELF64 little-endian
  RISC-V executables with one `_start`, panic handler and allocator provider;
  the entry lies in an executable LOAD segment.
- Static comparison preserves143 original production function signatures/bodies,
  all65 runtime test bodies,145 fixture/test functions,19 fixture field bodies,
  and14 valid local production projections. `League` retains its original three
  ordered fields after the minimal module-visibility adaptation. Peer and the
  five Router API files preserve their original bytes.

Local build output SHA-256 (paths and profiles are recorded with Cargo JSON;
debug hashes can differ across checkout paths because of debug information):

| Profile | Hub ELF SHA-256 |
| --- | --- |
| debug | `e8cd564754ec2a97d6267acbc7f8af84d42639429db1d6bf0b8f1304d59b5a61` |
| release | `6da2c1949153ada0d7a36e5189783b28aba2baba7e340d8c2bea2d1f0346a5e2` |

The first production host check failed with E0433/raw101 because the new server
manifest omitted the existing `hub-client` dependency used by the unchanged
`serve/bond.rs` body. The same genuine worker added only that local dependency;
normal Cargo lock refresh, host-library check and both RV profiles then passed.
The original failure remains recorded. A static-report parser initially read a
League import as a field; the corrected report restricts parsing to the struct
body, with both reports retained. Neither correction changed test assertions,
production runtime bodies, timeouts or reclamation criteria.

The initial public snapshot `67ad17644db0505ae7aa5b85e77078338463af0c`
contained109 tracked files instead of the frozen110: the coordination suite's
local `.gitignore` excluded its generated Cargo.lock. Public validation stopped
at this packaging check before Cargo ran. Root explicitly tracks the exact
already-validated lock bytes in the follow-up commit and repeats validation from
a new public checkout. The initial snapshot and failed proof remain preserved;
this correction changes release packaging and this record only.

Host transport/resource fixtures do not prove Kernel DMA reclaim. Actual Source
composition, Kernel `3aa0a155f825fbf377fc8d04009472ca67cbbf8c`, System runtime
`9001af65f97501dfa490c9397c61d593c871136d`, and SMP4/8 QEMU integration remain
pending at this provider publication. No1024 long matrix or full Driver migration
is claimed. Root retains raw command exits, actual artifact metadata, independent
review, source maps and immutable copied ELF evidence.
