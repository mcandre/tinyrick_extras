# OVERVIEW

fizzy's own compilation process is compatible with standard `cargo`. We wrap some common workflows with `tinyrick` tasks for convenience.

# BUILDTIME REQUIREMENTS

* [Rust](https://www.rust-lang.org/en-US/) 1.87.0+
* [cargo-audit](https://crates.io/crates/cargo-audit)
* [clippy](https://github.com/rust-lang-nursery/rust-clippy)
* [tinyrick](https://github.com/mcandre/tinyrick) (e.g., `cargo install tinyrick`)

# INSTALL BINARIES FROM LOCAL SOURCE

```console
$ tinyrick install
```

# UNINSTALL BINARIES

```console
$ tinyrick uninstall
```

# SECURITY AUDIT

```console
$ tinyrick audit
```

# LINT

```console
$ tinyrick lint
```

# TEST

```console
$ tinyrick test
```

# DOCUMENT API

```console
$ tinyrick doc
```

# PUBLISH

```console
$ tinyrick publish
```

# CLEAN

```console
$ tinyrick clean
```
