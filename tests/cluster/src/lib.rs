//! Standalone aggregate contract checks for the Driver Cluster registrations.
//!
//! These tests link the real `driver-cluster` declarations; no local `UnitFile`
//! or `ImageBuild` fixtures are defined here.

#[cfg(test)]
mod tests {
    use driver_cluster::{hub, router, IMAGE_BUILDS, UNITS};
    use unit::{Bootstrap, Dependency};

    #[test]
    fn units_are_hub_then_router() {
        assert_eq!(UNITS.len(), 2);
        assert_eq!(UNITS[0].name, "hub");
        assert_eq!(UNITS[1].name, "router");
        assert!(core::ptr::eq(UNITS[0], &hub::UNIT));
        assert!(core::ptr::eq(UNITS[1], &router::UNIT));
        assert!(UNITS[0].valid());
        assert!(UNITS[1].valid());
        assert_ne!(UNITS[0].name, UNITS[1].name);

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
        assert_eq!(IMAGE_BUILDS.len(), 2);

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

        assert_eq!(IMAGE_BUILDS[0].image, "hub");
        assert_eq!(IMAGE_BUILDS[1].image, "router");
        assert_eq!(IMAGE_BUILDS[0].image, UNITS[0].loader.image);
        assert_eq!(IMAGE_BUILDS[1].image, UNITS[1].loader.image);
        assert_eq!(
            (IMAGE_BUILDS[0].package, IMAGE_BUILDS[0].bin),
            ("hub-server", "hub")
        );
        assert_eq!(
            (IMAGE_BUILDS[1].package, IMAGE_BUILDS[1].bin),
            ("router-server", "router")
        );
        assert_ne!(IMAGE_BUILDS[0].image, IMAGE_BUILDS[1].image);
        assert_ne!(IMAGE_BUILDS[0].bin, IMAGE_BUILDS[1].bin);
        assert_ne!(IMAGE_BUILDS[0].manifest, IMAGE_BUILDS[1].manifest);
    }
}
