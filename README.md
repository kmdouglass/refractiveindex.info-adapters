# RIA: refractiveindex.info-adapters

[![docs.rs](https://img.shields.io/docsrs/ria)](https://docs.rs/ria/latest/lib_ria/)
[![Crates.io Version](https://img.shields.io/crates/v/ria)](https://crates.io/crates/ria)

Adapters to transform the refractiveindex.info database into single-file key/value stores.

`ria` provides two tools:

1. a Rust library containing the flattened `Store` datatype and methods for extracting the data, and
2. a command line tool to parse the [refractiveindex.info database](https://github.com/polyanskiy/refractiveindex.info-database) into a flat key/value store and write it to file.

## Quick start

### Install with Cargo

The CLI tool is an optional feature that can be installed from crates.io as

```console
cargo install ria --features cli
```

To install from this source code repository:

```console
cargo install --path . --features cli
```

### Create a single-file JSON store of the RefractiveIndex.info database

This assumes that you are currently inside the root folder of the refractiveindex.info-database repository. It will write a file called `results.dat` containing the data in JSON format.

```console
ria store
```

### Create a single-file bitcode store of the RefractiveIndex.info database

The database is in `refractiveindex.info-database/database`.

```console
ria -f bitcode store -p refractiveindex.info-database/database
```

### Create a single-file JSON store and include only keys in a file

The file should contain one key per line.

```console
ria store -p refractiveindex.info-database/database -i misc/cherry-initial-data.txt
```

### Validate a store

The file `results.dat` contains JSON data.

```console
ria -f json validate -i results.dat
```

### Get help

```console
ria --help
```

## Development

### Cloning this repository

Git provides two options to clone this repository, which contains the database as a submodule:

#### Recurse submodules

```console
# Assuming you're using SSH and not HTTPS

git clone --recurse-submodules git@github.com:kmdouglass/refractiveindex.info-adapters.git
```

#### Init and update

```console
# Assuming you're using SSH and not HTTPS

git clone git@github.com:kmdouglass/refractiveindex.info-adapters.git
cd refractiveindex.info-database
git submodule init
git submodule update
```

### Updating the database

```console
cd refractiveindex.info-database
git fetch
git merge origin/master
```

### Test and format

```console
cargo test --all-features
cargo fmt
```
