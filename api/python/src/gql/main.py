import typing as tp

from graphene import Field
from graphene import List
from graphene import ObjectType
from graphene import Int
from graphene import Schema
from graphene import String

from src.db.database import db_session
from src.db.models import EmailModel
from src.db.models import UserModel


class UserObject(ObjectType):
    id = Int()
    name = String()
    surname = String()
    age = Int()
    emails = List(lambda: EmailObject)

    @staticmethod
    def resolve_emails(root, info) -> tp.List[dict]:
        return root.emails


class EmailObject(ObjectType):
    id = Int()
    id_user = Int()
    email = String()
    user = Field(lambda: UserObject)

    @staticmethod
    def resolve_user(root, info) -> tp.Optional[dict]:
        return root.user


class Query(ObjectType):
    email = Field(EmailObject, email_id=Int())
    user = Field(UserObject, user_id=Int())
    users_by_min_age = List(UserObject, min_age=Int())

    @staticmethod
    def resolve_email(root, info, email_id=Int()) -> tp.Optional[EmailModel]:
        return db_session.query(EmailModel).filter(EmailModel.id == email_id).first()

    @staticmethod
    def resolve_user(root, info, user_id=Int()) -> tp.Optional[UserModel]:
        return db_session.query(UserModel).filter(UserModel.id == user_id).first()

    @staticmethod
    def resolve_users_by_min_age(root, info, min_age=Int()) -> tp.List[UserModel]:
        return db_session.query(UserModel).filter(UserModel.age >= min_age).all()


schema = Schema(query=Query)
