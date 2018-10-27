//! Build configuration

extern crate tinyrick;
extern crate tinyrick_extras;

/// Run clippy
fn clippy() {
  tinyrick_extras::clippy();
}

/// Run linters
fn lint() {
  tinyrick::deps(clippy);
}

/// Compile project
fn build() {
  tinyrick_extras::build();
}

/// Generate documentation
fn doc() {
  tinyrick_extras::build();
}

/// Run unit tests
fn unit_test() {
  tinyrick_extras::unit_test();
}

/// Run integration tests
fn integration_test() {
  assert!(
    tinyrick::exec_mut!("tinyrick", &["clean", "test", "install", "uninstall"])
      .current_dir("example")
      .status()
      .unwrap()
      .success()
  );
}

/// Run all tests
fn test() {
  tinyrick::deps(unit_test);
  tinyrick::deps(integration_test);
}

/// Publish to crate repository
fn publish() {
  tinyrick_extras::publish();
}

/// Run cargo clean
fn clean_cargo() {
  tinyrick_extras::clean_cargo();
}

/// Clean workspaces
fn clean() {
  tinyrick::deps(clean_cargo);
}

tinyrick::wubba_lubba_dub_dub!(
  test;
  clippy,
  lint,
  build,
  doc,
  unit_test,
  integration_test,
  publish,
  clean_cargo,
  clean
);
