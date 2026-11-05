# Driver Unit TOML intake

Base: `bb3427e179e87b640e0d51115a44544fae231252`.

The aggregate selects these published provider revisions through its gitlinks:

| Provider | Revision |
| --- | --- |
| Hub | `b1489c1754d34eb213c37be845c2581bbd4eb799` |
| Router | `14533071e586359989c71b64c50b556445c7ba56` |
| UART | `b9fabb22baeadd2e39f6bba5dc0fcf0a41939658` |
| RTC | `2c9caffb1019abff57f07017b4161db00db5abd4` |

Each provider owns its `unit.toml` and Cargo binaries. Recursive discovery can
read those files without a new central registry. Cluster remains an organization
layer. System is the boot Unit; Operator and Identity are System's internal
threads, so the provider documents do not register them as Units.

The existing `cluster/` Rust declarations and their Cargo dependency pins remain
unchanged. They are the legacy aggregate interface, separate from discovery of
the provider TOML at the gitlinks above. The older revision tables and validation
records describe that retained interface and its historical checkpoints.

## Validation on 2026-11-05

- Fetched each exact revision from its existing public `origin`. All four trees
  equal their previously verified local checkpoints, and all provider worktrees
  are clean. Each old local checkpoint is retained at
  `refs/backup/unit-toml-runtime-pre-public-20261105` in its provider repository;
  branch alignment used an atomic `update-ref` transaction with expected-old SHA.
- Recursive Git file discovery found exactly four provider `unit.toml` files.
  TOML parsing confirmed their IDs, image names, existing Cargo bins and source
  entry files. Hub has no Unit prerequisites, Router requires/follows Hub, and
  UART/RTC require/follow Hub and Router.
- With `/workspace/.vanllia-tools/env.sh` (`nightly-2026-10-09`), locked online
  and offline Cargo metadata were identical for the Driver workspace (34
  packages, three members) and `tests/cluster` (20 packages, one member). The
  first offline attempt failed because legacy `router.git@8a941627...` was not
  cached; locked online metadata populated it before the passing offline repeat.
  No Cargo manifests or locks changed. This checks the retained legacy graph,
  not a new TOML runtime closure.
- `git diff --check` passed.

Provider tests and executable builds were not repeated for this gitlink-only
intake. Guest, QEMU, SMP and runtime behavior were not validated. Old scenarios,
PCI, IOMMU and VirtIO block support remain outside this change.
