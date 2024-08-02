## Introduction

API created with Python code.

## Run

### Using sqlite db

#### Run sqlite without docker

Do you have the `contacts.sqlite3` file?

- Yes. Save it at `/tmp/` and run:

    ```bash
    make run-sqlite
    ```

- No. Create the `contacts.test.sqlite3` by running:

    ```bash
    make test
    ```

    And run the server with:

    ```bash
    make run-sqlite-test-db
    ```

You can make queries:

- Using Bruno. See [this folder](bruno/).
- Using the web browser, open the URL <http://127.0.0.1:5000/graphql>.

You have examples queries in the [Bruno folder](bruno/) and in the [test file](tests/unit/test_gql_schema.py).

You can run the app using docker, see the [makefile](makefile) commands. The URL is still the specified above.

#### Run sqlite with docker

First, copy the `contacts.sqlite3` file to the `/tmp` folder. See the `run sqlite without docker` to create the file if it does not exist.

After that:

```bash
make docker-build
make docker-run
```

See the `run sqlite without docker` to know how to make queries.

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
