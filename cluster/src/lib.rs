#![no_std]
//! Compatibility Driver Cluster declarations for Hub, Router, UART and RTC.
//!
//! Each Unit owns its declaration and runtime. This aggregate only selects,
//! reexports and orders the four real `UNIT`/`BUILD` declarations.
//!
//! Current Vanllia discovery reads provider-owned `unit.toml` files through the
//! aggregate's submodules, including PCI, IOMMU and VirtIO block. This legacy
//! Rust registry is not the complete TOML-discovered Unit collection.

pub use hub_declaration as hub;
pub use router_declaration as router;
pub use rtc_declaration as rtc;
pub use uart_declaration as uart;

pub static UNITS: &[&unit::UnitFile] = &[
    &hub::UNIT,
    &router::UNIT,
    &uart::UNIT,
    &rtc::UNIT,
];

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
    unit::ImageBuild {
        image: uart::BUILD.image,
        manifest: "units/uart/server/Cargo.toml",
        package: uart::BUILD.package,
        bin: uart::BUILD.bin,
        features: uart::BUILD.features,
        target: uart::BUILD.target,
        profile: uart::BUILD.profile,
        default_features: uart::BUILD.default_features,
    },
    unit::ImageBuild {
        image: rtc::BUILD.image,
        manifest: "units/rtc/server/Cargo.toml",
        package: rtc::BUILD.package,
        bin: rtc::BUILD.bin,
        features: rtc::BUILD.features,
        target: rtc::BUILD.target,
        profile: rtc::BUILD.profile,
        default_features: rtc::BUILD.default_features,
    },
];
