#![no_std]
//! Explicit Driver Cluster members for the current Hub slice.
//!
//! Router runtime/client, UART, RTC, PCI, IOMMU, virtio-blk and the device
//! supervisor remain pending; this is not a complete Driver Cluster.

pub use hub_declaration as hub;

pub static UNITS: &[&unit::UnitFile] = &[&hub::UNIT];

pub static IMAGE_BUILDS: &[unit::ImageBuild] = &[unit::ImageBuild {
    image: hub::BUILD.image,
    manifest: "units/hub/server/Cargo.toml",
    package: hub::BUILD.package,
    bin: hub::BUILD.bin,
    features: hub::BUILD.features,
    target: hub::BUILD.target,
    profile: hub::BUILD.profile,
    default_features: hub::BUILD.default_features,
}];
