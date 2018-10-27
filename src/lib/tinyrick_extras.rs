//! Premade tasks for common tinyrick workflows

extern crate tinyrick;

/// Run clippy
pub fn clippy() {
  tinyrick::exec!("cargo", &["clippy"]);
}

/// Compile project
pub fn build() {
  tinyrick::exec!("cargo", &["build"]);
}

/// Generate documentation
pub fn doc() {
  tinyrick::exec!("cargo", &["doc"]);
}

/// Install applications
pub fn install_binaries() {
  tinyrick::exec!("cargo", &["install", "--force", "--path", "."]);
}

/// Uninstall artifacts
pub fn uninstall() {
  tinyrick::exec!("cargo", &["uninstall"]);
}

/// Run unit tests
pub fn unit_test() {
  tinyrick::exec!("cargo", &["test"]);
}

/// Publish to crate repository
pub fn publish() {
  tinyrick::exec!("cargo", &["publish"]);
}

/// Run cargo clean
pub fn clean_cargo() {
  tinyrick::exec!("cargo", &["clean"]);
}
