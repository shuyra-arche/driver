//! `driver_shared::context` identity against the pinned public types.

const _: fn(
    env::TaskId,
    env::Wait,
) -> Result<driver_shared::context::Context, driver_shared::context::Step> =
    driver_shared::context::Context::open;

const _: fn(&driver_shared::context::Context) -> &system_client::operator::Capability =
    driver_shared::context::Context::capability;

const _: fn(
    &driver_shared::context::Context,
    env::PieToken,
    &str,
    env::Wait,
) = driver_shared::context::Context::plate;

const _: fn(
    &driver_shared::context::Context,
    u32,
    env::Wait,
) -> Result<router_client::Line, ()> = driver_shared::context::Context::line;

const _: driver_shared::context::Step = driver_shared::context::Step::Tree;

#[allow(dead_code)]
fn session_field(context: &driver_shared::context::Context) -> &ipc::session::Session {
    &context.session
}
