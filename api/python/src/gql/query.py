import typing as tp

from graphene import Field
from graphene import Int
from graphene import List
from graphene import ObjectType
from graphene import String
from sqlalchemy import or_

from src.db.database import db_session
from src.db.models import AddressModel
from src.db.models import EmailModel
from src.db.models import UserModel
from src.gql.types import EmailObject
from src.gql.types import UserObject


class Query(ObjectType):
    email = Field(EmailObject, email_id=Int())
    user = Field(UserObject, user_id=Int())
    search_user = List(UserObject, search_term=String())  # List field for search results

    @staticmethod
    def resolve_email(root, info, email_id=Int()) -> tp.Optional[EmailModel]:
        return db_session.query(EmailModel).filter(EmailModel.id == email_id).first()

    @staticmethod
    def resolve_user(root, info, user_id=Int()) -> tp.Optional[UserModel]:
        return db_session.query(UserModel).filter(UserModel.id == user_id).first()

    @staticmethod
    def resolve_search_user(root, info, search_term=String()) -> tp.Optional[tp.List[UserModel]]:
        """
        https://stackoverflow.com/questions/8561470/sqlalchemy-filtering-by-relationship-attribute
        """
        return (
            db_session.query(UserModel)
            .filter(
                or_(
                    UserModel.name.contains(search_term),
                    UserModel.surname.contains(search_term),
                    UserModel.emails.any(EmailModel.email.contains(search_term)),
                    UserModel.addresses.any(AddressModel.address.contains(search_term)),
                )
            )
            .all()
        )
