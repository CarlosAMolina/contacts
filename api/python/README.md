## Introduction

API created with Python code.

## Build

```bash
make docker-build
```

## Run

### Using sqlite db

#### Create sqlite db file

The file must be called `contacts.sqlite3`. Copy this file to the '/tmp' folder.

If you don't have this file, you can create the `contacts.test.sqlite3` file by running:

```bash
make test
```

#### Run API server without docker

If you are working with the `contacts.sqlite3` file, run:

```bash
make run-sqlite
```

If you are working with the `contacts.test.sqlite3` file, run:

```bash
make run-sqlite-test-db
```

#### Run API server with docker

You must work with the `contacts.sqlite3` file. Execute:

```bash
make docker-build
make docker-run
```

#### Make queries

You can make queries:

- Using Bruno. See [this folder](bruno/).
- Using the web browser, open the URL <http://127.0.0.1:5000/graphql>.

You have examples queries in the [Bruno folder](bruno/) and in the [test file](tests/unit/test_gql_schema.py).

You can run the app using docker, see the [makefile](makefile) commands. The URL is still the specified above.

## Alembic

Create initial files:

```bash
alembic init migrations
```

Files to modify:

- alembic.ini
- migrations/env.py

Create alembic version with the current model changes:

```bash
alembic revision --autogenerate -m "Initial migration"
```

Apply modifications to the database:

```bash
make alembic-upgrade-sqlite
```

## Resources

Alembic tutorial, [chapter 7 of this book](https://www.manning.com/books/microservice-apis), [book's code](https://github.com/abunuwas/microservice-apis/tree/master).
