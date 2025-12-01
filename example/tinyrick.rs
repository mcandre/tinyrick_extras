//! Build configuration

extern crate tinyrick;
extern crate tinyrick_extras;

/// Generate documentation
fn doc() {
    tinyrick_extras::build();
}

/// Security audit
fn audit() {
    tinyrick_extras::cargo_audit();
}

/// banner generates artifact labels.
fn banner() -> String {
    format!("{}-{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

/// Run cargo check
fn cargo_check() {
    tinyrick_extras::cargo_check();
}

/// Clean workspaces
fn clean() {
    tinyrick::deps(clean_ports);
    tinyrick_extras::clean_cargo();
}

/// Clean precompiled binaries
fn clean_ports() {
    assert!(
        tinyrick::exec_mut!("crit", &["-c"])
            .status()
            .unwrap()
            .success()
    );
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

/// Prepare cross-platform release media.
fn port() {
    let b = &banner();
    tinyrick_extras::crit(vec!["-b".to_string(), b.to_string()]);
    tinyrick_extras::chandler(".crit/bin", b);
}

/// Publish to crate repository
fn publish() {
    tinyrick_extras::publish();
}

/// CLI entrypoint
fn main() {
    tinyrick::phony!(clean);

    tinyrick::wubba_lubba_dub_dub!(
        build;
        doc,
        audit,
        cargo_check,
        clean,
        clean_ports,
        clippy,
        install,
        lint,
        port,
        publish,
        test,
        uninstall
    );
}
