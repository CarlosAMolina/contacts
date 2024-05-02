## Introduction

API created with Python code.

## Run

### Using sqlite db

First create the db file:

```bash
make test
```

```bash
make run-sqlite
```

## Alembic

Create initial files:

```bash
alembic init migrations
```

Files to modify:

```bash
# alembic.ini
sqlalchemy.url = sqlite:////tmp/contacts.sqlite3
```

```bash
# migrations/env.py
from src.db.models import Base
target_metadata = Base.metadata
```

Create alembic version with the current model changes:

```bash
alembic revision --autogenerate -m "Initial migration"
```

Apply modifications to the database:

```bash
alembic upgrade heads
```

## Resources

Alembic tutorial, [chapter 7 of this book](https://www.manning.com/books/microservice-apis).
