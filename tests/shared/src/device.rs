//! `driver_shared::device` identity against the pinned public types.

const _: fn(
    &system_client::operator::Face,
    env::Reason,
    env::Wait,
) -> Result<driver_shared::device::Hub, driver_shared::fail::Fail> =
    driver_shared::device::Hub::find;

const _: fn(
    &driver_shared::device::Hub,
    &system_client::operator::Face,
    &driver_shared::device::Ask,
    env::Reason,
    env::Wait,
) -> Result<hub_api::Deed, driver_shared::fail::Fail> = driver_shared::device::Hub::claim;

const _: fn(env::PieToken) -> Result<driver_shared::device::Device, ()> =
    driver_shared::device::Device::open;

const _: fn(&driver_shared::device::Device) -> resource::dock::View =
    driver_shared::device::Device::view;

/// `Ask` keeps its five public fields in their original types and order.
#[allow(dead_code)]
fn ask_fields(
    ask: &driver_shared::device::Ask,
) -> (
    &'static str,
    Option<&'static str>,
    &env::PieKind,
    &env::Access,
    &env::Policy,
) {
    (ask.class, ask.name, &ask.kind, &ask.access, &ask.policy)
}
