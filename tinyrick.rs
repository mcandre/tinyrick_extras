//! Build configuration

use tinyrick::*;
use tinyrick_extras;

use std::fs;
use std::path;
use std::process;

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
    deps!(clean_example);
    deps!(clean_cargo);
}

/// Clean cargo
#[task]
fn clean_cargo() {
    tinyrick_extras::clean_cargo();
}

/// Clean example
#[task]
fn clean_example() {
    let pth_cargo_lock = path::Path::new("example").join("Cargo.lock");
    let pth_target = path::Path::new("example").join("target");
    let _ = fs::remove_file(pth_cargo_lock);
    let _ = fs::remove_dir_all(pth_target);
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

/// Integration test
#[task]
fn itest() {
    process::Command::new("tinyrick")
        .current_dir("example")
        .status()
        .unwrap();
}

/// Validate documentation and run linters
#[task]
fn lint() {
    deps!(cargo_check);
    deps!(clippy);
    deps!(doc);
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

/// Doc, lint, and run tests
#[task]
fn test() {
    tinyrick_extras::unit_test();
}

/// CLI entrypoint
fn main() {
    run();
}
