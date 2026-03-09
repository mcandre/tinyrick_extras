# DEVELOPMENT GUIDE

tinyrick_extras follows standard, cargo based operations for compiling and unit testing Rust code.

For advanced operations, such as linting, we further supplement with some software industry tools.

# BUILDTIME REQUIREMENTS

* a UNIX-like environment (e.g. [WSL](https://learn.microsoft.com/en-us/windows/wsl/))
* POSIX compliant [make](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/make.html)
* [Rust](https://www.rust-lang.org/en-US/)
* Provision additional dev tools with `make`

## Recommended

* [ASDF](https://asdf-vm.com/) 0.18 (run `asdf reshim` after provisioning)

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
