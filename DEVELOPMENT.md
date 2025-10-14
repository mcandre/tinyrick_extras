# OVERVIEW

tinyrick_extras' own compilation process is compatible with standard cargo. We wrap some common workflows with tinyrick tasks for convenience.

# BUILDTIME REQUIREMENTS

* [GNU make](https://www.gnu.org/software/make/) 3+
* [Rust](https://www.rust-lang.org/en-US/)
* Provision additional dev tools with `make [-j 4 --output-sync]`

## Recommended

* a UNIX-like environment (e.g. [WSL](https://learn.microsoft.com/en-us/windows/wsl/))
* [ASDF](https://asdf-vm.com/) 0.18 (run `asdf reshim` after provisioning)
* [cargo-cache](https://crates.io/crates/cargo-cache)
* [direnv](https://direnv.net/) 2

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
