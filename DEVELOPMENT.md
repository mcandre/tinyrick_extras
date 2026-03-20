# DEVELOPMENT

tinyrick_extras follows standard, cargo based operations for compiling and unit testing Rust code.

For advanced operations, such as linting, we further supplement with some software industry tools.

# DEV ENVIRONMENT

## Prerequisities

* [make](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/make.html)
* [Rust](https://www.rust-lang.org/en-US/)
* Provision additional dev tools with `make -f install.mk`

## Recommended

* [asdf](https://asdf-vm.com/)

## Postinstall

Register `~/.cargo/bin` to `PATH` environment variable.

# TASKS

We automate engineering tasks.

## Build

```sh
tinyrick
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
