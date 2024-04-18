import typing as tp

from graphene import Field
from graphene import Int
from graphene import List
from graphene import ObjectType
from graphene import String

from src.db.database import db_session
from src.db.models import EmailModel
from src.db.models import UserModel
from src.gql.types import EmailObject
from src.gql.types import UserObject


class Query(ObjectType):
    email = Field(EmailObject, email_id=Int())
    user = Field(UserObject, user_id=Int())
    users_by_min_age = List(UserObject, min_age=Int())
    search_user = List(UserObject, q=String())  # List field for search results

    @staticmethod
    def resolve_email(root, info, email_id=Int()) -> tp.Optional[EmailModel]:
        return db_session.query(EmailModel).filter(EmailModel.id == email_id).first()

    @staticmethod
    def resolve_user(root, info, user_id=Int()) -> tp.Optional[UserModel]:
        return db_session.query(UserModel).filter(UserModel.id == user_id).first()

    @staticmethod
    def resolve_users_by_min_age(root, info, min_age=Int()) -> tp.List[UserModel]:
        return db_session.query(UserModel).filter(UserModel.age >= min_age).all()

    @staticmethod
    def resolve_search_user(root, info, **args):
        q = args.get("q")
        return db_session.query(UserModel).filter(UserModel.name.contains(q)).all()
