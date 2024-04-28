from sqlalchemy.orm import scoped_session
from sqlalchemy.orm import sessionmaker
import sqlalchemy as sa

from src.db import data
from src.db import models


_db_path_name = "/tmp/contacts.sqlite3"
_DB_URL = f"sqlite:///{_db_path_name}"
_engine = sa.create_engine(_DB_URL)
db_session = scoped_session(sessionmaker(autocommit=False, autoflush=False, bind=_engine))
# TODO close the session


def prepare_db():
    _drop_db_tables(_engine)
    _create_db_tables(_engine)
    _insert_db_data(db_session)


def _drop_db_tables(engine):
    models.Base.metadata.drop_all(bind=engine)


def _create_db_tables(engine):
    models.Base.metadata.create_all(bind=engine)


def _insert_db_data(db_session):
    data_variable_and_model = {
        "addresses": models.AddressModel,
        "categories": models.CategoryModel,
        "discord": models.DiscordModel,
        "emails": models.EmailModel,
        "facebook": models.FacebookModel,
        "github": models.GitHubModel,
        "instagram": models.InstagramModel,
        "linkedin": models.LinkedinModel,
        "nicknames": models.NicknameModel,
        "notes": models.NoteModel,
        "phones": models.PhoneModel,
        "telegram": models.TelegramModel,
        "twitter": models.TwitterModel,
        "urls": models.UrlModel,
        "users_categories": models.UserCategoryModel,
        "users": models.UserModel,
        "wallapop": models.WallapopModel,
    }
    rows_to_insert = []
    for data_variable, model in data_variable_and_model.items():
        model_rows_to_insert = [model(**row) for row in getattr(data, data_variable)]
        rows_to_insert += model_rows_to_insert
    for row in rows_to_insert:
        db_session.add(row)
    db_session.commit()
