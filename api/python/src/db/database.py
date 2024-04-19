from sqlalchemy.orm import scoped_session
from sqlalchemy.orm import sessionmaker
import sqlalchemy as sa

from src.db import data
from src.db import models


_db_path_name = "/tmp/contacts.sqlite3"
_url = f"sqlite:///{_db_path_name}"
_engine = sa.create_engine(_url)
db_session = scoped_session(sessionmaker(autocommit=False, autoflush=False, bind=_engine))
# TODO close the session


def prepare_db():
    print(f"Preparing DB: {_db_path_name}")
    _drop_db_tables(_engine)
    _create_db_tables(_engine)
    _insert_db_data(db_session)


def _drop_db_tables(engine):
    models.Base.metadata.drop_all(bind=engine)


def _create_db_tables(engine):
    models.Base.metadata.create_all(bind=engine)


def _insert_db_data(db_session):
    addresses = [models.AddressModel(**row) for row in data.addresses]
    discord = [models.DiscordModel(**row) for row in data.discord]
    users = [models.UserModel(**row) for row in data.users]
    emails = [models.EmailModel(**row) for row in data.emails]
    rows_to_insert = addresses + discord + users + emails
    for row in rows_to_insert:
        db_session.add(row)
    db_session.commit()
