# OVERVIEW

smartpointers' own compilation process is compatible with standard cargo. We wrap some common workflows with tinyrick tasks for convenience.

# BUILDTIME REQUIREMENTS

* [Rust](https://www.rust-lang.org/en-US/) 1.30+

## Recommended

* [clippy](https://github.com/rust-lang-nursery/rust-clippy)
* [tinyrick](https://github.com/mcandre/tinyrick) (e.g., `cargo install tinyrick`)
* [coreutils](https://www.gnu.org/software/coreutils/coreutils.html)
* [Docker](https://www.docker.com/)

# BUILD: DOC, LINT, TEST, COMPILE

```console
$ tinyrick [build]
```

# PUBLISH

```console
$ tinyrick publish
```

# CLEAN

```console
$ tinyrick clean
```
