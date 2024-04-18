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
        search_term_to_use = _get_search_term_to_use(search_term)
        return (
            db_session.query(UserModel)
            .filter(
                or_(
                    UserModel.name.like(search_term_to_use),
                    UserModel.surname.like(search_term_to_use),
                    UserModel.emails.any(EmailModel.email.like(search_term_to_use)),
                    UserModel.addresses.any(AddressModel.address_unicode.like(search_term_to_use)),
                )
            )
            .all()
        )


def _get_search_term_to_use(string: str) -> str:
    result = string
    result = _get_string_to_search_replace_accents(result)
    result = f"%{result}%"
    return result


def _get_string_to_search_replace_accents(string: str) -> str:
    replacements = [
        "á",
        "Á",
        "é",
        "É",
        "í",
        "Í",
        "ó",
        "Ó",
        "ú",
        "Ú",
    ]
    result = string
    for character in replacements:
        result = result.replace(character, "%")
    return result
