## Introduction

API created with Python code.

## Run

### Using sqlite db

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
- Using the web browser, open [this url](http://127.0.0.1:5000/graphql).

You have examples queries in the [Bruno folder](bruno/) and in the [test file](tests/unit/test_gql_schema.py).

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

Alembic tutorial, [chapter 7 of this book](https://www.manning.com/books/microservice-apis).
