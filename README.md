# refractiveindex.info-adapters

Adapters to transform the refractiveindex.info database into other forms

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
git merge origin/main
```
