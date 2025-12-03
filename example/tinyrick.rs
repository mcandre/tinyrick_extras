//! Build configuration

use tinyrick::*;
use tinyrick_extras;

/// Build: Doc, lint, test, and compile
#[default_task]
fn build() {
    deps!(test);
    tinyrick_extras::build();
}

/// Generate documentation
#[task]
fn doc() {
    tinyrick_extras::build();
}

/// Security audit
#[task]
fn audit() {
    tinyrick_extras::cargo_audit();
}

/// banner generates artifact labels.
fn banner() -> String {
    format!("{}-{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

/// Run cargo check
#[task]
fn cargo_check() {
    tinyrick_extras::cargo_check();
}

/// Clean workspaces
#[task]
fn clean() {
    deps!(clean_ports);
    tinyrick_extras::clean_cargo();
}

/// Clean precompiled binaries
#[task]
fn clean_ports() {
    exec!("crit", "-c");
}

/// Run clippy
#[task]
fn clippy() {
    tinyrick_extras::clippy();
}

/// Validate documentation and run linters
#[task]
fn lint() {
    deps!(doc);
    deps!(clippy);
}

/// Install binaries
#[task]
fn install() {
    tinyrick_extras::install_binaries();
}

/// Uninstall binaries
#[task]
fn uninstall() {
    tinyrick_extras::uninstall_binaries();
}

/// Doc, lint and run tests
#[task]
fn test() {
    deps!(lint);
    deps!(install);
    tinyrick_extras::unit_test();
    exec!("fizzbuzz");
}

/// Prepare cross-platform release media.
#[task]
fn port() {
    let b = &banner();
    tinyrick_extras::crit(&vec!["-b", b]);
    tinyrick_extras::chandler(&format!(".crit{}bin", std::path::MAIN_SEPARATOR_STR), b);
}

/// Publish to crate repository
#[task]
fn publish() {
    tinyrick_extras::publish();
}

/// CLI entrypoint
fn main() {
    run();
}
