# Driver aggregate provenance

Current architecture is Cluster aggregation with fixed Unit Git submodules.
Vanllia fixes the Cluster; the Cluster fixes each Unit. The previous Driver
snapshot8e6de5a77aa52184bb6f7fbbcbc961f779e94629 remains in public Git history.
Its complete frozen original file map is retained below as historical provenance.
The aggregate has no second API implementation outside its fixed Unit submodules.
API sources inside units/hub and units/router are owned by those Unit repositories.

| Submodule | Public repository | Complete revision |
| --- | --- | --- |
| units/hub | https://github.com/shuyra-arche/hub.git | `daa23b2611fee0c503bc6bd31a16121d97698ad6` |
| units/router | https://github.com/shuyra-arche/router.git | `94fec223f3c5df6c99a49527a65d016d88de3af4` |

Hub extends its existing API history fromcccef3f325198c8390007ae88b064bae0f21f93e;
all35 canonical API files and12 canonical test files remain byte-exact (six Rust
files, two manifests, two locks and two .gitignore files, counted from Gitcccef).
Its public
clean checkout proved140 files,84 distinct host cases(65 runtime+19 canonical),
four host libraries and actual RV debug/release Hub executables. Original143
production functions,65 runtime cases,145 fixture/test functions,19 field bodies
and14 local production projections retain their accepted contract. Hub's own
PROVENANCE keeps the original API provenance verbatim and adds76 Source293 paths,
original hashes/authors and no invented license. The old copied coordination
suite is replaced by the original canonical suite; no assertion was deleted.

Router currently owns only the original five pure API files, publicly checked
byte-exact to Source293. Router client/server/declaration/runtime remain pending
in Source. Its PROVENANCE records those five paths/hashes and original author.

Shared support/ is still the single pure Peer implementation, unchanged from
public Driver8e6. Runtime Unit and Source consumers intentionally pin the named
driver-support package at immutable8e6, preserving System Face5a and avoiding a
repository release cycle. The aggregate's own support library is not a Hub
runtime dependency; actual resolve closures are checked. Hub API remainscccef,
Router API94fec, System API9e/client-support5a, Kernel3aa/System runtime9001.
No System-peer publication, nominal SDK cascade, Source backlink or path override.

The pure driver-cluster interface registers only migrated Hub and its explicit
BUILD, with one manifest prefix units/hub/server/Cargo.toml. Units own their
independent workspaces. Source/vanllia still selects the combined declaration
set and Deployment. Other Driver runtimes and mixed device-recovery/system-shape
acceptance remain Source-owned until their own slices. Cluster/public recursive
proof and real Kernel3aa/System9001 SMP4/8 DMA/reclaim integration are separate
pending gates; host transport/resource mocks are not Kernel proof. No full Driver
or Router migration is claimed. The Source root had no project LICENSE; none is
invented, and original Unit histories/headers/authorship remain preserved.

---

## Retained historical original-snapshot provenance (Driver8e6)

# Provenance

Original Source: https://github.com/anreonyr/sqware.git
Frozen revision: `293c15fc97a9d9a4295810d10fe804e4cdec63c4`.

This first Driver Cluster slice owns the Hub client/server/declaration and its
65 runtime tests, shared `Peer`, and the pure Router API prerequisite. Router
runtime/client and the other Driver Units remain pending. The actual combined
composition, selected Deployment and cross-Cluster acceptance stay in Source
and the future Vanllia Integration.

Root copied the original files byte-for-byte before the genuine DeepSeek worker
adapted package boundaries. Public Hub API history remains canonical at
https://github.com/shuyra-arche/hub/tree/cccef3f325198c8390007ae88b064bae0f21f93e;
this repository does not introduce a second active Hub API. System client/support
and all other public dependencies retain the full revisions in Cargo manifests
and locks. The old unpublished Hub/System-peer staging trees are retained by Root
and are not selected by this delivery.

Boundary adaptations use `crate::core`, public `system-support`, and
`driver-support::peer`, plus the public Execution/Mold entry and linker adapter.
The sole original `League` type is split from `serve/mod.rs` to
`serve/league.rs`; its three ordered fields keep their original types, with the
minimal `pub(super)` visibility required by that split. Runtime bodies, field
order, authorization, DMA/fence ordering, lifetimes, limits, errors and codec
quirks are preserved. The Hub declaration uses the same UnitFile and explicit
repository-relative build coordinates. The server's original call to
`hub_client::activate` is wired to the same-repository client package.

The 21 handoff, 39 retention and 5 capability-import runtime assertions retain
their original signatures, bodies and fixture functions. Their production-source
projections remain inside this repository. Five capability-import fixture types
are split into one-type files while retaining their public fixture paths.
The ten canonical API coordination tests are recorded separately and do not
replace those 65 runtime assertions. Mixed device-recovery and system-shape
tests remain Source-owned. Host mocks do not prove actual Kernel DMA reclaim.

## Original source-history authors

- Serial Device <anreonyr@gmail.com>
- anreonyr <88045926+anreonyr@users.noreply.github.com>
- anreonyr <anreonyr@gmail.com>

The Source root has no project LICENSE. This snapshot invents no license and
does not rewrite Source or previously published provider history. Existing file
headers remain in the preserved content.

## Frozen original file map

These hashes identify original pre-adaptation bytes in the public Source commit
above, not a claim that adapted files have the same whole-file hash. The League
and capability-import type splits described above add destination files.

| Source path | Initial copied ownership | Original SHA-256 |
| --- | --- | --- |
| `programs/src/service/hub/client/Cargo.toml` | `units/hub/client/Cargo.toml` | `827d1a0b802189e8e7ac5e56f1bca89a02865b6c7f5d45f5d64d496d4e3bd7e3` |
| `programs/src/service/hub/client/src/activation_client.rs` | `units/hub/client/src/activation_client.rs` | `461dcf49a85427041aa010a874f62fb476ce317903b494c4392128bfa4c10b7b` |
| `programs/src/service/hub/client/src/client.rs` | `units/hub/client/src/client.rs` | `a7672dc4759296f427aa02ad0c91d91945d62d3e96be531b396802ababa19977` |
| `programs/src/service/hub/client/src/debug.rs` | `units/hub/client/src/debug.rs` | `229562fb0a5f4046cabf6dac363f936d583c41b7e659a54799a6547d7f194244` |
| `programs/src/service/hub/client/src/lib.rs` | `units/hub/client/src/lib.rs` | `e7aab0012673911cc638e2a7cd007da7d70a41478f3c5f873a11cf26089ab0f1` |
| `programs/src/service/hub/mod.rs` | `units/hub/server/src/lib.rs` | `57f0f357116087f8660583b7c6ab5226ace2e96e25b7ee0b1e0adc3a91df259a` |
| `programs/src/service/hub/fault.rs` | `units/hub/server/src/fault.rs` | `72eb7363ebc19983f68a9e90f8584c8cdb5413da63720ea19f4f9fb3a659292f` |
| `programs/src/service/hub/core/admission.rs` | `units/hub/server/src/core/admission.rs` | `13f33d5cc8dd677fbbe7f86994e9a4c7edcc972a1687a6adeee81326399be00a` |
| `programs/src/service/hub/core/admissions.rs` | `units/hub/server/src/core/admissions.rs` | `1a42904083eb28f614fd60c563ed665c3190e5f0a61d7871412f2177a88ef632` |
| `programs/src/service/hub/core/binding.rs` | `units/hub/server/src/core/binding.rs` | `89d5b3792aa9fb9491cd726c61c954e558fb5cea22defc0efa961ed76679f38b` |
| `programs/src/service/hub/core/bindings.rs` | `units/hub/server/src/core/bindings.rs` | `cf366bc732f972b42deb711a9d904be253e5962824f566e2628b1e3f7e031056` |
| `programs/src/service/hub/core/cell.rs` | `units/hub/server/src/core/cell.rs` | `12d4d37109719cae9c8384025c965d49932f70feebd9e106b327c9532ecfb9ef` |
| `programs/src/service/hub/core/claim_request.rs` | `units/hub/server/src/core/claim_request.rs` | `e735a840302804c57e3cbb75a789caa689cf2db60fdda2b14252291849ff8435` |
| `programs/src/service/hub/core/enrollment.rs` | `units/hub/server/src/core/enrollment.rs` | `df452248fde587012343ec4d7fe5585be9247d8055bac5cc6b7141b70884cb7e` |
| `programs/src/service/hub/core/enrollment_effects.rs` | `units/hub/server/src/core/enrollment_effects.rs` | `e906dd4a56433100606defd5eadc6ee890bb1ef7087ec79a6c7631c269f69a13` |
| `programs/src/service/hub/core/enrollment_step.rs` | `units/hub/server/src/core/enrollment_step.rs` | `8876f9399389f23ad96380738ef5a68f074834cfe9f6c91fd3df1052916488bb` |
| `programs/src/service/hub/core/enrollments.rs` | `units/hub/server/src/core/enrollments.rs` | `bf527f2248b03e9ceebd9f7b1e8792bf331cd7520f0d1213e41e196d0729dff4` |
| `programs/src/service/hub/core/entry.rs` | `units/hub/server/src/core/entry.rs` | `0044177f836b44d31cc880239a6c5b15034d78b4c63663d79b668f87ad60a22c` |
| `programs/src/service/hub/core/league.rs` | `units/hub/server/src/core/league.rs` | `191f324ddd398faf4cfbf9fb1bd32f61cd1b6fff978ba003e942319f35da1d7d` |
| `programs/src/service/hub/core/league_step.rs` | `units/hub/server/src/core/league_step.rs` | `1bc325ff4ea7d00662750885742014d2c1308422eac0e123ec6f1e2fdfe5d388` |
| `programs/src/service/hub/core/leagues.rs` | `units/hub/server/src/core/leagues.rs` | `33e0f22486f84cca76d918b63c0fa31be3769260065b5ad8423657a3dc5aee34` |
| `programs/src/service/hub/core/ledger.rs` | `units/hub/server/src/core/ledger.rs` | `6af366b62761f014c7407d1602d773186bfe29373ac6109d554a17d76fdc07bd` |
| `programs/src/service/hub/core/mod.rs` | `units/hub/server/src/core/mod.rs` | `411409070324492c6549342a2fc5395a210d32fe54f82d7cf90367117f8f919b` |
| `programs/src/service/hub/core/owner.rs` | `units/hub/server/src/core/owner.rs` | `35a83bbddc20c0cc99f08b564a4078f2270f08863a0582046d60258fb1dbbe30` |
| `programs/src/service/hub/serve/binding_rpc.rs` | `units/hub/server/src/serve/binding_rpc.rs` | `441887fb2d956f96da901bfbe454ad18148f08b48d17c15aa8de3b5148f94dde` |
| `programs/src/service/hub/serve/bond.rs` | `units/hub/server/src/serve/bond.rs` | `d337f763b5dafc3a97f4033c00192a79ce1988221b17e98d023bdb2ba61183e4` |
| `programs/src/service/hub/serve/claim.rs` | `units/hub/server/src/serve/claim.rs` | `7f28c012a8a08d889de8edb2f7352827cdd25e57ed7306d469555f0d6b5071aa` |
| `programs/src/service/hub/serve/coordination.rs` | `units/hub/server/src/serve/coordination.rs` | `7aa87bc54c90f4bf4c86f3e1efae2f6ed5eb4f78d43d95a0e9f1d463633ddfa7` |
| `programs/src/service/hub/serve/directory.rs` | `units/hub/server/src/serve/directory.rs` | `e2bc7a162850e205ad53d0e828e6f11516352c3fa92f6ec8ffde9e28ca85dbc9` |
| `programs/src/service/hub/serve/dma.rs` | `units/hub/server/src/serve/dma.rs` | `65aaefccdcdcb803a6e7b575939266e06ad3f55e617e295e6f01265522b74e3c` |
| `programs/src/service/hub/serve/enrollment_failure.rs` | `units/hub/server/src/serve/enrollment_failure.rs` | `96b8d247d28b85cef022cb9bc6ea6b1fc7e482da443e468a5e9e0b0b423c1325` |
| `programs/src/service/hub/serve/execution.rs` | `units/hub/server/src/serve/execution.rs` | `1378e33b524098463ab16d1f310d196d60e7f86c349d6c866d7ee152c475cdd1` |
| `programs/src/service/hub/serve/grant.rs` | `units/hub/server/src/serve/grant.rs` | `fe8d406f54d78b2ad770782a10a2849b7c10e7bc3630b5073d918911f51213a9` |
| `programs/src/service/hub/serve/launch.rs` | `units/hub/server/src/serve/launch.rs` | `0064512f7b2f9a1a4fac51845c54adce10c019b51050263edada716a69031576` |
| `programs/src/service/hub/serve/launch_phase.rs` | `units/hub/server/src/serve/launch_phase.rs` | `f092d061a8d76519e6c054bcd120c17c83dc88df140b187284ba0708bd92a402` |
| `programs/src/service/hub/serve/launch_reply.rs` | `units/hub/server/src/serve/launch_reply.rs` | `72bd08d1d4364f455186b92797ff09a418c5298723ca760e683e9b01ab22702a` |
| `programs/src/service/hub/serve/list.rs` | `units/hub/server/src/serve/list.rs` | `884d405a90a9b5186fcb3bc2752a2385edda80d1244e2a82ec8f296b7ccc7ec7` |
| `programs/src/service/hub/serve/mod.rs` | `units/hub/server/src/serve/mod.rs` | `280df984c78fb13e7f06e165d28e93e327f70db8c153c6e77bc2a6fa9c882c51` |
| `programs/src/service/hub/serve/peers.rs` | `units/hub/server/src/serve/peers.rs` | `7b19472a0c7d3249911f43dea193cf534d46a80ab86d4fa343980ede882d9db2` |
| `programs/src/service/hub/serve/return_reply.rs` | `units/hub/server/src/serve/return_reply.rs` | `47599b9abc49485d5280e25b32a536deaf1616cc4404fb491eb99f281b94b6ec` |
| `programs/src/service/hub/serve/returned.rs` | `units/hub/server/src/serve/returned.rs` | `5e45efc9cd956f07b04965c66034fe9774c469a3142102619a394941648a5b08` |
| `programs/src/service/hub/serve/sweep.rs` | `units/hub/server/src/serve/sweep.rs` | `22e8b2df4f71dddda98624fb05060b752b06af79f06172f4098d77ae0db4726d` |
| `programs/src/service/hub/main.rs` | `units/hub/server/src/main.rs` | `09f351ca1bb9bbf36856774ad081ba2cf3c1288669c3d30524e0282c1c7a6b16` |
| `crates/composition/src/units/hub.rs` | `units/hub/declaration/src/lib.rs` | `969e3858d66c7297b6fbfa4b1f0c262a4fc59c58e838c0dcf04f076e4fa9ae74` |
| `programs/src/driver/shared/peer.rs` | `support/src/peer.rs` | `ca28129462031dcd96bbd59118f28c762eeca4bcf827c99a563ab47779cb4a5f` |
| `programs/src/driver/router/api/Cargo.toml` | `units/router/api/Cargo.toml` | `eb3e4c344473b5000246edf469d71b7d130a09bf80535902717cb038473d9379` |
| `programs/src/driver/router/api/src/frame.rs` | `units/router/api/src/frame.rs` | `c83b48f513a855ba9d85985d8efd4051f8429e4da6ae169e12af69c1d59957b4` |
| `programs/src/driver/router/api/src/lib.rs` | `units/router/api/src/lib.rs` | `857f873e88084552761167eddafe05ecf27cd30e93df3fdb76781750bf19c52b` |
| `programs/src/driver/router/api/src/marks.rs` | `units/router/api/src/marks.rs` | `29f7e70f52438f952c9483bc8a5126b4cdd6ce17925388b1a66fa6de102a048c` |
| `programs/src/driver/router/api/src/declaration.rs` | `units/router/api/src/declaration.rs` | `5d5795333f1a6a915cb0d2afaeb1991fef0e4ecace98010f550f2e2dc188abdb` |
| `programs/tests/hub-handoff/Cargo.lock` | `units/hub/tests/handoff/Cargo.lock` | `37d3283b7c05c3a9ba778dd9a91fd66172f5bf193e2dae87f1a9fd7db86221e9` |
| `programs/tests/hub-handoff/Cargo.toml` | `units/hub/tests/handoff/Cargo.toml` | `714d82bb51823c4378f0714dba7f007fe5436036acddf4b4e325bf9319d1f1e1` |
| `programs/tests/hub-handoff/backend.rs` | `units/hub/tests/handoff/backend.rs` | `ab864212f56b7d1cf0870a4d24946805257c1a12ead804feaa7868f92667dc05` |
| `programs/tests/hub-handoff/cases.rs` | `units/hub/tests/handoff/cases.rs` | `9a4ef4b8f7ce3e099d48a3f2a0cd68924badc5197d54c5e0dcf313b516e1b5b8` |
| `programs/tests/hub-handoff/coord_backend.rs` | `units/hub/tests/handoff/coord_backend.rs` | `4cd92aef4469c1dacf246e3c844af298aadeaa623f6e1d34bc26bfb6e4bb753e` |
| `programs/tests/hub-handoff/enrollment.rs` | `units/hub/tests/handoff/enrollment.rs` | `3261f8a504581cf0162cb1fb7a6a06aab445fbfb8baf5106b3ad598a869d5cd8` |
| `programs/tests/hub-handoff/lib.rs` | `units/hub/tests/handoff/lib.rs` | `e4ac15abf55ee667586f15923920a422f1c594c37bcf32ef66244115c9ca05f6` |
| `programs/tests/hub-handoff/raw.rs` | `units/hub/tests/handoff/raw.rs` | `de1919d0d3456d73d692e7c22a6c433b65fa677116d87e55952615a0ab5feb97` |
| `programs/tests/hub-handoff/rpc.rs` | `units/hub/tests/handoff/rpc.rs` | `db6961a882ef342a5b3afa966974b0396398cad7411367a3df28dc580b809d65` |
| `programs/tests/hub-handoff/backend/kind.rs` | `units/hub/tests/handoff/backend/kind.rs` | `3760d20c318cbfd90aae866a9097b6b12437012baaf754ba471308f5bd585bbd` |
| `programs/tests/hub-handoff/coord_backend/state.rs` | `units/hub/tests/handoff/coord_backend/state.rs` | `91afc89049b57a8b115d84aa152994ba6a304c8e027800da2906ceed74d89568` |
| `programs/tests/hub-handoff/raw/hole.rs` | `units/hub/tests/handoff/raw/hole.rs` | `22334d35996ed848d6dc4925d1d314a2b87eb6326cf17de7eb3ae915d4ed8605` |
| `programs/tests/hub-handoff/rpc/request.rs` | `units/hub/tests/handoff/rpc/request.rs` | `23f11c5bf8dd6ddc1185059498cd07fea0cc309f1f0563f36c6d93de3d6fa0c9` |
| `programs/tests/hub-handoff/serve/adapter_cases.rs` | `units/hub/tests/handoff/serve/adapter_cases.rs` | `08fa22def45c28cce492908131288c704a20afea942cfe60e67c47fb2fcb0eaf` |
| `programs/tests/hub-handoff/serve/mod.rs` | `units/hub/tests/handoff/serve/mod.rs` | `5ac917edaad15b975b38e088e4c7d05dd07a164075e5c592195dee1052e3e72f` |
| `programs/tests/hub-handoff/serve/peers.rs` | `units/hub/tests/handoff/serve/peers.rs` | `260b64a0cd438efbec996607551c6bc2440cd6d46fcfec122b8375a6a2eeef29` |
| `programs/tests/hub-handoff/serve/peers/control_face.rs` | `units/hub/tests/handoff/serve/peers/control_face.rs` | `b0ca3c3cc254090d757f03436c68ef55b328940433d0803aeae95138337756ce` |
| `programs/tests/hub-handoff/serve/peers/coord_face.rs` | `units/hub/tests/handoff/serve/peers/coord_face.rs` | `d8c7a603844dfb6b9ab42763105bd517b9cdf0decc93bea61c8a60b5ec709d8b` |
| `programs/tests/hub-handoff/serve/peers/instance.rs` | `units/hub/tests/handoff/serve/peers/instance.rs` | `09714c2c51c8467351b515c0d629f2deec70ffcd3fba77431d11df44f2e52d58` |
| `programs/tests/retention/Cargo.lock` | `units/hub/tests/retention/Cargo.lock` | `4472c0efeaeba727164e1ce30957303b618ecc090ab9fe1f1ff3fd146b72937d` |
| `programs/tests/retention/Cargo.toml` | `units/hub/tests/retention/Cargo.toml` | `b3fa4b73ded58c5b42a83c61f726a904e318b6203a488e801a7f4520120ccebd` |
| `programs/tests/retention/enrollment.rs` | `units/hub/tests/retention/enrollment.rs` | `d69735d472b62384935bd0430d922b676c92139709c6a7a0de9ed5357a46d42b` |
| `programs/tests/retention/execution.rs` | `units/hub/tests/retention/execution.rs` | `cf86b4e8d296f9c166007554d2b82799d78e54a4e9748255dd8a375f424a9ba4` |
| `programs/tests/retention/lib.rs` | `units/hub/tests/retention/lib.rs` | `31a9b48efd5cba41b378b7c6b43b6f55d411b7472807ad495e388d29198dfe8c` |
| `programs/tests/retention/protocol.rs` | `units/hub/tests/retention/protocol.rs` | `917c7eff98c72b3e4685805b537725aac34637e2cf37636104b9a198314c0a8d` |
| `programs/tests/retention/enrollment/boundary.rs` | `units/hub/tests/retention/enrollment/boundary.rs` | `cb5090a5082b85116a670cab378f295776f521a2251388630a806828b1300d5b` |
| `programs/tests/retention/enrollment/effects.rs` | `units/hub/tests/retention/enrollment/effects.rs` | `e4f442178e75162edea6207146f4ac1790e260ac870cfad62b226d66ed6eff4e` |
| `programs/tests/retention/enrollment/failure.rs` | `units/hub/tests/retention/enrollment/failure.rs` | `54d461d822de14485cd58789e3be6b520d82dca0169637dc2dffdc2fe83a3cb2` |
| `programs/tests/capability-import/Cargo.lock` | `units/hub/tests/capability-import/Cargo.lock` | `3a62d164e4b60e8f712e9b5c167451dca3ed95b13d012c1f9ec41ee3dc36b4fd` |
| `programs/tests/capability-import/Cargo.toml` | `units/hub/tests/capability-import/Cargo.toml` | `744200a959b006debdb77187045e2b5f278989d29384a99512258e0c36e5d003` |
| `programs/tests/capability-import/hub_serve.rs` | `units/hub/tests/capability-import/hub_serve.rs` | `fe34b20e7656049b5e9453adec1b4824133fc8b7ae6c6d71c39a3c5f00a3a7b8` |
| `programs/tests/capability-import/lib.rs` | `units/hub/tests/capability-import/lib.rs` | `734427e2d29a2d731628a444935ace0a3fac471a95c1985dd04d0fe81a86cd22` |
