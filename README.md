## Introduction

Project to manage contacts

## Docker

### Initial configuration

#### Docker network

The network must be created before running the containers:

```bash
docker network create --subnet 172.20.0.0/16 contacts-network
```

[Resource](https://dev.to/rizkyrajitha/connect-api-and-a-database-with-docker-network-299g).

## Python version

### API Python

Too build and run the API, see the README file in the `api/python` folder.

### CLI Python

Too build and run the CLI, see the README file in the `cli-python` folder.

### VPS configuration Python

You can create an alias to search contacts quickly:

```bash
# Contacts
c () {
    cd ~/contacts/cli-python/
    make docker-search-term term="$@"
    cd - > /dev/null
}
ci () {
    cd ~/contacts/cli-python/
    make docker-search-id term="$@"
    cd - > /dev/null
}
```

Example of use, search term: `c moli`.

Example of use, search ID: `ci 9`.

## Rust version

### Build

Docker is installed in rootless mode.

If you are not in Debian, you can build the app to this platform running:

```bash
docker pull rust
make build_for_debian
```

The executable file can be found at `contacts/target/release/`.

### Run db

To run the Docker database, read [this file](https://github.com/CarlosAMolina/postgresql/blob/main/docker/README.md).

The db makefile is makefile-db.

Command example:

```bash
make -f makefile-db run-docker
```

### VPS configuration Rust

You can create an alias to search contacts quickly:

```bash
alias c='cd ~/contacts/cli/ && ./cli $1'
# Example of use: c moli
```

### Run search contacts

```bash
make run
```

### Front

#### Configuration front

The `contacts.js` file must be edited, change the `urlPrefix` constant with it's real value.

### Tests

The integration tests require:

- `sqlx` must be installed: `cargo install sqlx-cli`.
- The database must be running.

Run the tests with:

```bash
make test
```

## VPS configuration

Rotless containers must be able to run after SSH logout. [Resource](https://stackoverflow.com/questions/71372713/rootless-mode-docker-daemon-not-running-after-logging-back-in-ssh).
