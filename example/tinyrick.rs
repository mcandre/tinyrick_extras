//! Build configuration

use std::path;

extern crate tinyrick;
extern crate tinyrick_extras;

/// Generate documentation
fn doc() {
  tinyrick_extras::build();
}

/// Run clippy
fn clippy() {
  tinyrick_extras::clippy();
}

/// Validate documentation and run linters
fn lint() {
  tinyrick::deps(doc);
  tinyrick::deps(clippy);
}

/// Install binaries
fn install() {
  tinyrick_extras::install_binaries();
}

/// Uninstall binaries
fn uninstall() {
  tinyrick_extras::uninstall_binaries();
}

/// Doc, lint and run tests
fn test() {
  tinyrick::deps(lint);
  tinyrick::deps(install);

  tinyrick_extras::unit_test();
  tinyrick::exec!("fizzbuzz");
}

/// Build: Doc, lint, test, and compile
fn build() {
  tinyrick::deps(test);
  tinyrick_extras::build();
}

/// banner generates artifact labels.
fn banner() -> String {
    format!("{}-{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

fn archive() {
    tinyrick_extras::archive(path::Path::new(".crit").join("bin").display().to_string(), banner().to_string());
}

/// Prepare cross-platform release media.
fn port() {
    tinyrick_extras::crit(vec!["-b".to_string(), banner()]);
    tinyrick::deps(archive);
}

/// Publish to crate repository
fn publish() {
  tinyrick_extras::publish();
}

/// Clean workspaces
fn clean() {
  tinyrick_extras::clean_cargo();
}

/// CLI entrypoint
fn main() {
  tinyrick::phony!(clean);

  tinyrick::wubba_lubba_dub_dub!(
    build;
    doc,
    clippy,
    lint,
    install,
    uninstall,
    test,
    archive,
    port,
    publish,
    clean
  );
}
