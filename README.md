# RIA: refractiveindex.info-adapters

![docs.rs](https://img.shields.io/docsrs/ria)
![Crates.io Version](https://img.shields.io/crates/v/ria)

Adapters to transform the refractiveindex.info database into single-file key/value stores.

`ria` provides two tools:

1. a command line tool to parse the [refractiveindex.info database](https://github.com/polyanskiy/refractiveindex.info-database) into a flat key/value store and write it to file, and
2. a Rust library containing the flattened `Store` datatype and methods for extracting the data.

## Quick start

### Install with Cargo

```console
cargo install ria
```

### Create a single-file JSON store of the RefractiveIndex.info database

This assumes that you are currently inside the root folder of the refractiveindex.info-database repository. It will write a file called `results.dat` containing the data in JSON format.

```console
ria store
```

### Create a single-file bitcode store of the RefractiveIndex.info database

The database is in `refractiveindex.info-database/`.

```console
ria -f bitcode store -p refractiveindex.info-database/
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
cargo test
cargo fmt
```
