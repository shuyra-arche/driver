#![no_std]
//! Explicit Driver Cluster declarations for the migrated Hub and Router slices.
//!
//! Only the Hub and Router `UNIT`/`BUILD` declarations are registered here.
//! Router server integration, UART, RTC, PCI, IOMMU, virtio-blk and the device
//! supervisor remain pending; this is not a complete Driver Cluster.

pub use hub_declaration as hub;
pub use router_declaration as router;

pub static UNITS: &[&unit::UnitFile] = &[&hub::UNIT, &router::UNIT];

pub static IMAGE_BUILDS: &[unit::ImageBuild] = &[
    unit::ImageBuild {
        image: hub::BUILD.image,
        manifest: "units/hub/server/Cargo.toml",
        package: hub::BUILD.package,
        bin: hub::BUILD.bin,
        features: hub::BUILD.features,
        target: hub::BUILD.target,
        profile: hub::BUILD.profile,
        default_features: hub::BUILD.default_features,
    },
    unit::ImageBuild {
        image: router::BUILD.image,
        manifest: "units/router/server/Cargo.toml",
        package: router::BUILD.package,
        bin: router::BUILD.bin,
        features: router::BUILD.features,
        target: router::BUILD.target,
        profile: router::BUILD.profile,
        default_features: router::BUILD.default_features,
    },
];
