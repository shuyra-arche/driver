# PCI, IOMMU and VirtIO block intake

Base: `12eeb6e147d873eaeeb5fc8dbb8193d8e2253a0b`, the published
`codex/driver-cluster` default branch after Unit TOML intake.

The aggregate adds three provider-owned submodules at their reviewed heads:

| Path | Repository | Fixed revision |
| --- | --- | --- |
| `units/pci` | `shuyra-arche/driver-pci` | `b2c2e5d7a5fbd0b1572273dbbffb56bd91b15009` |
| `units/iommu` | `shuyra-arche/driver-iommu` | `3d210b4da522ee0c7427265ce0a0d8946e124b21` |
| `units/virtio-blk` | `shuyra-arche/driver-virtio-block` | `58569a4f3033edc699d1ba02ce2f4f660a2480dc` |

Existing Hub, Router, UART and RTC gitlinks remain as recorded in
[UNIT_TOML_INTAKE.md](UNIT_TOML_INTAKE.md). Each new revision was fetched from its
public origin; no provider source, declaration or test is copied into this
aggregate. The `virtio-blk` directory follows the existing Unit ID and image;
its repository retains the `driver-virtio-block.git` URL.

The first aggregate checkpoint `12601f0ee59d1a452665d90a6e5f1c1afcca4b80`
selected VirtIO block `fecc0147065e1f00f5a30f77490a369582cfc2f2` and is retained
in history. Its follow-up selected above pins the accepted PCI/IOMMU revisions
in VirtIO block's standalone Cargo graph and adds the provider-owned
`test.virtio-blk-discovery` native probe. VirtIO block's production server,
declaration and `unit.toml` bytes are unchanged between those two revisions.

Vanllia discovers provider `unit.toml` documents recursively through these
gitlinks. PCI and IOMMU require/follow Hub. VirtIO block requires/follows Hub,
PCI and IOMMU. No new central Unit registry is needed. The legacy `cluster/`
Rust interface, its Cargo manifests and locks remain unchanged.

PCI owns both `service/pci/entry` and `service/pci/config` in one task. IOMMU
owns its independent `service/iommu/entry` authentication. VirtIO block owns
`service/device/coord`. These are provider policies; the aggregate adds no
authentication aliases or capabilities. BARs and DMA source remain dynamic:
VirtIO block has no static Supply bindings. PCI consumes System's existing
`platform/ecam` and `platform/aperture`; IOMMU consumes existing
`memory/retention` and `platform/iommu` resources. Vanllia must select a System
revision whose metadata declares those resources. This intake creates no
platform supply and makes no claim that a particular machine has an IOMMU.

## Aggregate validation on 2026-11-05

- All seven provider Unit documents parse, have distinct IDs and images, and
  resolve to one existing Cargo binary and source entry each. The same scan
  also finds PCI's new `test.pci-inspection` and VirtIO block's new
  `test.virtio-blk-discovery` probes and their binaries: nine distinct Unit/bin
  pairs in total. These are provider-owned validation, not migrated legacy
  scenarios.
- The three new checkouts match the fixed revisions above. Existing provider
  pins and all provider worktrees remain unchanged and clean.
- Under `nightly-2026-10-09`, locked online and offline Cargo metadata for the
  retained Driver workspace were byte-identical: 34 packages and three members.
  The initial offline command lacked historical Git dependencies; locked online
  resolution populated the cache before the passing offline repeat. Cargo
  manifests and locks did not change. Both staged and worktree `git diff --check`
  passed.
- No `.agents/skills` directory or `SKILL.md` was present in the aggregate or
  new provider checkouts. Their `AGENTS.md` instructions were read; this intake
  uses independent namer, worker and reviewer roles.
- Independent review confirmed the six-file scope, seven exact clean provider
  checkouts, unchanged dependency and supply boundaries, passing diff checks
  and the same locked offline metadata graph at the first checkpoint.
- Follow-up review confirmed the two-file VirtIO revision/document delta,
  nine distinct Unit/bin pairs, unchanged production bytes and correct accepted
  PCI/IOMMU Cargo pins. The new probe is transient and requires/follows
  `virtio-blk`; its saved source and normalized serial-log hashes also match.

This repository change only selects source revisions. Combined Cargo resolution,
RV64 artifacts and QEMU acceptance belong to the Vanllia composition update.
It establishes no physical-hardware, real DMA or block-read/write result.
Legacy tests and scenarios are not migrated, and no Kernel cluster is added.
