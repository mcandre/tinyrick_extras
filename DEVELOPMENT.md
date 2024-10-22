# OVERVIEW

tinyrick_extras' own compilation process is compatible with standard cargo. We wrap some common workflows with tinyrick tasks for convenience.

# BUILDTIME REQUIREMENTS

* [GNU](https://www.gnu.org/software/make/) / [BSD](https://man.freebsd.org/cgi/man.cgi?make(1)) make
* [Rust](https://www.rust-lang.org/en-US/) 1.75.0+
* Provision additional dev tools with `make -j 4`

## Recommended

* [ASDF](https://asdf-vm.com/) 0.10 (run `asdf reshim` after provisioning)
* [cargo-cache](https://crates.io/crates/cargo-cache)
* [direnv](https://direnv.net/) 2
* POSIX compatible [tar](https://pubs.opengroup.org/onlinepubs/7908799/xcu/tar.html)

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
