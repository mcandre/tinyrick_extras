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

/// Run cargo check
fn cargo_check() {
    tinyrick_extras::cargo_check();
}

/// Clean workspaces
fn clean() {
    tinyrick_extras::clean_cargo();
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
        clippy,
        install,
        lint,
        publish,
        test,
        uninstall
    );
}
