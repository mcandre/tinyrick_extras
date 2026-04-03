# DEVELOPMENT

We follow standard, `cargo` based operations for compiling and unit testing Rust code.

For advanced operations, such as linting, we further supplement with some software industry tools.

# PREREQUISITES

* [Rust](https://www.rust-lang.org/en-US/)
* [cargo-audit](https://crates.io/crates/cargo-audit)
* [clippy](https://github.com/rust-lang-nursery/rust-clippy)
* [tinyrick](https://github.com/mcandre/tinyrick)

## Postinstall

Register `~/.cargo/bin` to `PATH` environment variable.

# TASKS

We automate engineering tasks.

## Build

```sh
tinyrick
```

## Install

```sh
tinyrick install
```

# Uninstall

```sh
tinyrick uninstall
```

## Security Audit

```sh
tinyrick audit
```

## Lint

```sh
tinyrick lint
```

## Test

```sh
tinyrick test
```

## Generate API Docs

```sh
tinyrick doc
```

## Publish Crate

```sh
tinyrick publish
```

## Clean Workspace

```sh
tinyrick clean
```
