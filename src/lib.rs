//! Premade tasks for common tinyrick workflows

extern crate tinyrick;

/// Build all binaries
pub fn build() {
    tinyrick::deps!(build_debug);
    tinyrick::deps!(build_release);
}

/// Build debug binaries
pub fn build_debug() {
    tinyrick::exec!("cargo", "build");
}

/// Build release binaries
pub fn build_release() {
    tinyrick::exec!("cargo", "build", "--release");
}

/// Run cargo audit
pub fn cargo_audit() {
    tinyrick::exec!("cargo", "audit");
}

/// Run cargo check
pub fn cargo_check() {
    tinyrick::exec!("cargo", "check");
}

/// Run cargo clean
pub fn clean_cargo() {
    tinyrick::exec!("cargo", "clean");
}

/// Run clippy
pub fn clippy() {
    tinyrick::exec!("cargo", "clippy");
}

/// Generate cross-platform binaries.
pub fn crit(args: &[&str]) {
    tinyrick::exec("crit", args);
}

/// Generate Rust API documentation
pub fn doc() {
    tinyrick::exec!("cargo", "doc");
}

/// Install applications
pub fn install_binaries() {
    tinyrick::exec!("cargo", "install", "--force", "--path", ".");
}

/// Publish to crate repository
pub fn publish() {
    tinyrick::exec!("cargo", "publish");
}

/// Run rustfmt
pub fn rustfmt() {
    tinyrick::exec!("cargo", "fmt");
}

/// Uninstall artifacts
pub fn uninstall_binaries() {
    tinyrick::exec!("cargo", "uninstall");
}

/// Run unit tests
pub fn unit_test() {
    tinyrick::exec!("cargo", "test");
}
