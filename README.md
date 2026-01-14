# tinyrick_extras: common tasks for tinyrick projects

[![Crates.io Downloads (latest version)](https://img.shields.io/crates/dv/tinyrick_extras?label=crate%20downloads)](https://crates.io/crates/tinyrick_extras) [![docs.rs](https://img.shields.io/docsrs/tinyrick_extras)](https://docs.rs/tinyrick_extras/latest/tinyrick_extras/) [![Lint](https://github.com/mcandre/tinyrick_extras/actions/workflows/lint.yml/badge.svg)](https://github.com/mcandre/tinyrick_extras/actions/workflows/lint.yml) [![license](https://img.shields.io/badge/license-BSD-3)](LICENSE.md) [![Donate](https://img.shields.io/badge/GUMROAD-36a9ae?style=flat&logo=gumroad&logoColor=white)](https://mcandre.gumroad.com/)

# SUMMARY

tinyrick_extras provides prebaked tasks for common Rust software project developer needs.

# EXAMPLE

```console
$ cd example

$ tinyrick
running 1 test
test smoketest ... ok
...
Value(94)
Buzz
Fizz
Value(97)
Value(98)
Fizz
Buzz
```

# ABOUT

tinyrick_extras defines some common tasks, such as unit tests, linting, generating API documentation, publishing packages, installing and uninstalling packages, for your [tinyrick](https://github.com/mcandre/tinyrick) projects. Boom. Take what works for your build workflow, leave the rest.

Check out the [example](example) project.

# RUNTIME REQUIREMENTS

* [Rust](https://www.rust-lang.org/en-US/)
* [tinyrick](https://github.com/mcandre/tinyrick)

## Recommended

* [ASDF](https://asdf-vm.com/) 0.18 (run `asdf reshim` after each Rust application binary installation)
* [cargo-cache](https://crates.io/crates/cargo-cache)
* [chandler](https://github.com/mcandre/chandler)
* [crit](https://github.com/mcandre/crit) ports Rust applications
* [direnv](https://direnv.net/) 2
* [tuggy](https://github.com/mcandre/tuggy)

# CONTRIBUTING

For more details on developing tinyrick_extras itself, see [DEVELOPMENT.md](DEVELOPMENT.md).
