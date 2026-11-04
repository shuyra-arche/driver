//! Host-only identity checks for the provider `driver-shared` package.
//!
//! The items below only force this package's public signatures, public fields
//! and failure type to unify with the exact public Git revisions pinned in
//! `shared/Cargo.toml`. Nothing here calls a session, Face, Hub, device or
//! kernel operation, and no Source path or shadow Core is imported.

pub mod context;
pub mod device;
pub mod fail;
