# OVERVIEW

smartpointers' own compilation process is compatible with standard cargo. We wrap some common workflows with tinyrick tasks for convenience.

# BUILDTIME REQUIREMENTS

* [Rust](https://www.rust-lang.org/en-US/) 1.30+
* [clippy](https://github.com/rust-lang-nursery/rust-clippy)
* [coreutils](https://www.gnu.org/software/coreutils/coreutils.html)
* [Docker](https://www.docker.com/)

# LINT

```console
$ tinyrick lint
```

# TEST

```console
$ tinyrick test
```

# GENERATE DOCUMENTATION

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
