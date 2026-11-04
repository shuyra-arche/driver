//! `driver_shared::fail` identity plus pure `Exit`/`Report` reporting checks.
//!
//! `env::Reason` is the public `usize` exit-code alias, and `execution::boot`'s
//! `Report`/`Exit` path is pure host code (`Report::parts` just reads the stored
//! reason and note). Nothing here calls a session, Face, Hub, device, kernel or
//! syscall, and no test mocks a Core type.

use driver_shared::fail::Fail;
use env::Reason;
use execution::boot::{Exit, Report};

const _: fn(Reason, &'static str) -> Fail = Fail::at;

const _: for<'a> fn(&'a Fail) -> Report<'a> = <Fail as Exit>::report;

/// A nonempty note carries the exact reason and text through `Report::parts`.
#[test]
fn nonempty_note_preserves_reason_and_text() {
    for &code in &[0usize, 1, 0xDEAD_BEEF, usize::MAX] {
        let fail = Fail::at(code, "device open failed");
        let (reason, note) = fail.report().parts();
        assert_eq!(reason, code);
        assert_eq!(note, Some("device open failed"));
    }
}

/// An empty note keeps the reason but reads back as `None`, not `Some("")`.
#[test]
fn empty_note_preserves_reason_and_reports_none() {
    for &code in &[0usize, 1, usize::MAX] {
        let fail = Fail::at(code, "");
        let (reason, note) = fail.report().parts();
        assert_eq!(reason, code);
        assert_eq!(note, None::<&str>);
    }
}

/// `?`-style `Result<(), Fail>::Err` reports the failure's code and text.
#[test]
fn result_err_preserves_failure_code_and_text() {
    let failed: Result<(), Fail> = Err(Fail::at(7, "claim"));
    let (reason, note) = failed.report().parts();
    assert_eq!(reason, 7);
    assert_eq!(note, Some("claim"));
}

/// `Result<(), Fail>::Ok` is the success path: `EXIT_OK` and no note.
#[test]
fn result_ok_reports_exit_ok_without_note() {
    let passed: Result<(), Fail> = Ok(());
    let (reason, note) = passed.report().parts();
    assert_eq!(reason, env::EXIT_OK);
    assert_eq!(note, None::<&str>);
}
