## Introduction

Project to manage contacts

## Build

Docker is installed in rootless mode.

If you are not in Debian, you can build the app to this platform running:

```bash
docker pull rust
make build_for_debian
```

The executable file can be found at `contacts/target/release/`.

### Docker network

The network must be created before running the containers:

```bash
docker network create --subnet 172.20.0.0/16 contacts-network
```

<https://dev.to/rizkyrajitha/connect-api-and-a-database-with-docker-network-299g>

## Run db

To run the Docker database, read [this file](https://github.com/CarlosAMolina/postgresql/blob/main/docker/README.md).

The db makefile is makefile-db.

## VPS configuration

Rotless containers must be able to run after SSH logout.

Resource: <https://stackoverflow.com/questions/71372713/rootless-mode-docker-daemon-not-running-after-logging-back-in-ssh>

## Run search contacts

```bash
make run
```

### Search term

```bash
make search_term
```

## Front

## Configuration front

The `contacts.js` file must be edited, change the `urlPrefix` constant with it's real value.

## Tests

The integration tests require:

- `sqlx` must be installed: `cargo install sqlx-cli`.
- The database must be running.

Run the tests with:

```bash
make test
```

## Resources

Tutorial:

<https://docs.rs/csv/latest/csv/tutorial/>
