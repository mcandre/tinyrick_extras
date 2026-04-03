//! Build configuration

use tinyrick::*;
use tinyrick_extras;

/// Security audit
#[task]
fn audit() {
    tinyrick_extras::cargo_audit();
}

/// Build: Compile
#[default_task]
fn build() {
    tinyrick_extras::build();
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

/// Run clippy
#[task]
fn clippy() {
    tinyrick_extras::clippy();
}

/// Generate documentation
#[task]
fn doc() {
    tinyrick_extras::build();
}

/// Install binaries
#[task]
fn install() {
    tinyrick_extras::install_binaries();
}

/// Validate documentation and run linters
#[task]
fn lint() {
    deps!(doc);
    deps!(clippy);
    deps!(rustfmt);
}

/// Publish to crate repository
#[task]
fn publish() {
    tinyrick_extras::publish();
}

/// Run rustfmt
#[task]
fn rustfmt() {
    tinyrick_extras::rustfmt();
}

/// Doc, lint and run tests
#[task]
fn test() {
    deps!(lint);
    deps!(install);
    tinyrick_extras::unit_test();
    exec!("fizzbuzz");
}

/// Uninstall binaries
#[task]
fn uninstall() {
    tinyrick_extras::uninstall_binaries();
}

/// CLI entrypoint
fn main() {
    run();
}
