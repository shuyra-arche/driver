//! Standalone aggregate contract checks for the Driver Cluster registrations.
//!
//! These tests link the real `driver-cluster` declarations; no local `UnitFile`
//! or `ImageBuild` fixtures are defined here.

#[cfg(test)]
mod tests {
    use std::path::Path;

    use driver_cluster::{hub, router, rtc, uart, IMAGE_BUILDS, UNITS};
    use unit::{Bootstrap, Dependency};

    #[test]
    fn units_are_hub_router_uart_rtc() {
        assert_eq!(UNITS.len(), 4);
        assert_eq!(UNITS[0].name, "hub");
        assert_eq!(UNITS[1].name, "router");
        assert_eq!(UNITS[2].name, "uart");
        assert_eq!(UNITS[3].name, "rtc");
        assert!(core::ptr::eq(UNITS[0], &hub::UNIT));
        assert!(core::ptr::eq(UNITS[1], &router::UNIT));
        assert!(core::ptr::eq(UNITS[2], &uart::UNIT));
        assert!(core::ptr::eq(UNITS[3], &rtc::UNIT));
        assert!(UNITS[0].valid());
        assert!(UNITS[1].valid());
        assert!(UNITS[2].valid());
        assert!(UNITS[3].valid());
        for (index, unit) in UNITS.iter().enumerate() {
            for previous in &UNITS[..index] {
                assert_ne!(unit.name, previous.name);
                assert_ne!(unit.loader.image, previous.loader.image);
            }
        }

        let after = UNITS[1]
            .control
            .after
            .expect("Router declares startup dependencies");
        let operator_position = after
            .iter()
            .position(|dependency| {
                matches!(
                    dependency,
                    Dependency::Bootstrap(Bootstrap::Operator)
                )
            })
            .expect("Router follows the Operator bootstrap");
        let hub_position = after
            .iter()
            .position(|dependency| matches!(dependency, Dependency::Unit("hub")))
            .expect("Router follows Hub");
        assert!(operator_position < hub_position);

        assert_operator_hub_router(UNITS[2], "UART");
        assert_operator_hub_router(UNITS[3], "RTC");
    }

    fn assert_operator_hub_router(unit: &unit::UnitFile, label: &str) {
        let after = unit
            .control
            .after
            .unwrap_or_else(|| panic!("{label} declares startup dependencies"));
        let operator_position = after
            .iter()
            .position(|dependency| {
                matches!(
                    dependency,
                    Dependency::Bootstrap(Bootstrap::Operator)
                )
            })
            .unwrap_or_else(|| panic!("{label} follows the Operator bootstrap"));
        let hub_position = after
            .iter()
            .position(|dependency| matches!(dependency, Dependency::Unit("hub")))
            .unwrap_or_else(|| panic!("{label} follows Hub"));
        let router_position = after
            .iter()
            .position(|dependency| matches!(dependency, Dependency::Unit("router")))
            .unwrap_or_else(|| panic!("{label} follows Router"));
        assert!(operator_position < hub_position);
        assert!(hub_position < router_position);
    }

    fn build_matches_owner(
        registered: &unit::ImageBuild,
        owner: &unit::ImageBuild,
        manifest: &str,
    ) {
        assert_eq!(registered.image, owner.image);
        assert_eq!(registered.package, owner.package);
        assert_eq!(registered.bin, owner.bin);
        assert_eq!(registered.features, owner.features);
        assert_eq!(registered.target, owner.target);
        assert_eq!(registered.profile, owner.profile);
        assert_eq!(registered.default_features, owner.default_features);
        assert_eq!(registered.manifest, manifest);
        assert_eq!(owner.manifest, "server/Cargo.toml");
    }

    #[test]
    fn builds_route_to_real_servers() {
        assert_eq!(IMAGE_BUILDS.len(), 4);

        build_matches_owner(
            &IMAGE_BUILDS[0],
            &hub::BUILD,
            "units/hub/server/Cargo.toml",
        );
        build_matches_owner(
            &IMAGE_BUILDS[1],
            &router::BUILD,
            "units/router/server/Cargo.toml",
        );
        build_matches_owner(
            &IMAGE_BUILDS[2],
            &uart::BUILD,
            "units/uart/server/Cargo.toml",
        );
        build_matches_owner(
            &IMAGE_BUILDS[3],
            &rtc::BUILD,
            "units/rtc/server/Cargo.toml",
        );

        assert_eq!(IMAGE_BUILDS[0].image, "hub");
        assert_eq!(IMAGE_BUILDS[1].image, "router");
        assert_eq!(IMAGE_BUILDS[2].image, "uart");
        assert_eq!(IMAGE_BUILDS[3].image, "rtc");
        assert_eq!(IMAGE_BUILDS[0].image, UNITS[0].loader.image);
        assert_eq!(IMAGE_BUILDS[1].image, UNITS[1].loader.image);
        assert_eq!(IMAGE_BUILDS[2].image, UNITS[2].loader.image);
        assert_eq!(IMAGE_BUILDS[3].image, UNITS[3].loader.image);
        assert_eq!(
            (IMAGE_BUILDS[0].package, IMAGE_BUILDS[0].bin),
            ("hub-server", "hub")
        );
        assert_eq!(
            (IMAGE_BUILDS[1].package, IMAGE_BUILDS[1].bin),
            ("router-server", "router")
        );
        assert_eq!(
            (IMAGE_BUILDS[2].package, IMAGE_BUILDS[2].bin),
            ("uart-server", "uart")
        );
        assert_eq!(
            (IMAGE_BUILDS[3].package, IMAGE_BUILDS[3].bin),
            ("rtc-server", "rtc")
        );
        assert_ne!(IMAGE_BUILDS[0].image, IMAGE_BUILDS[1].image);
        assert_ne!(IMAGE_BUILDS[0].bin, IMAGE_BUILDS[1].bin);
        assert_ne!(IMAGE_BUILDS[0].manifest, IMAGE_BUILDS[1].manifest);

        for (index, build) in IMAGE_BUILDS.iter().enumerate() {
            for previous in &IMAGE_BUILDS[..index] {
                assert_ne!(build.image, previous.image);
                assert_ne!(build.bin, previous.bin);
                assert_ne!(build.manifest, previous.manifest);
            }
        }

        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        for build in IMAGE_BUILDS {
            assert!(
                root.join(build.manifest).is_file(),
                "manifest must exist: {}",
                build.manifest
            );
        }
    }
}
