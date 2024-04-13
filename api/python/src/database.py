from pathlib import Path

from sqlalchemy.orm import scoped_session
from sqlalchemy.orm import sessionmaker
import sqlalchemy as sa

from src import models


_db_path_name = "/tmp/contacts.sqlite3"
_url = f"sqlite:///{_db_path_name}"
_engine = sa.create_engine(_url)
_db_session = scoped_session(sessionmaker(autocommit=False, autoflush=False, bind=_engine))


def init_db():
    if _exists_db_file(_db_path_name):
        print(f"DB already exists: {_db_path_name}. Deleting")
        _delete_file_path(Path(_db_path_name))
    print(f"Start creating DB: {_db_path_name}")
    _create_db(_engine)
    _insert_db_data(_db_session, _users_data)


_users_data = [
    {
        "id": 1,
        "name": "John",
        "surname": "Doe",
    },
    {
        "id": 2,
        "name": "Jane",
    },
]


def _exists_db_file(db_path_name: str) -> bool:
    return Path(db_path_name).is_file()


def _delete_file_path(path: Path):
    path.unlink()


def _create_db(engine):
    models.Base.metadata.create_all(bind=engine)


def _insert_db_data(db_session, users_data: list):
    users = [
        models.User(
            id=user_data["id"],
            name=user_data["name"],
            surname=user_data.get("surname"),
        )
        for user_data in users_data
    ]
    for row in users:
        db_session.add(row)
    db_session.commit()
