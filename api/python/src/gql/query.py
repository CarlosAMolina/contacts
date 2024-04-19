import typing as tp

from graphene import Field
from graphene import Int
from graphene import List
from graphene import ObjectType
from graphene import String
from sqlalchemy import or_

from src.db.database import db_session
from src.db.models import AddressModel, DiscordModel
from src.db.models import EmailModel
from src.db.models import UserModel
from src.gql.types import EmailObject
from src.gql.types import UserObject
from src.utils.unicode import get_string_unicode


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
        search_term_unicode = get_string_unicode(search_term)
        return (
            db_session.query(UserModel)
            .filter(
                or_(
                    UserModel.name_unicode.contains(search_term_unicode),
                    UserModel.surname_unicode.contains(search_term_unicode),
                    UserModel.emails.any(EmailModel.email_unicode.contains(search_term_unicode)),
                    UserModel.addresses.any(AddressModel.address_unicode.contains(search_term_unicode)),
                    UserModel.discord.any(DiscordModel.user_name_unicode.contains(search_term_unicode)),
                    UserModel.discord.any(DiscordModel.discriminator.contains(search_term_unicode)),
                    UserModel.discord.any(DiscordModel.alias_unicode.contains(search_term_unicode)),
                    UserModel.discord.any(DiscordModel.global_name_unicode.contains(search_term_unicode)),
                    UserModel.discord.any(DiscordModel.legacy_user_name_unicode.contains(search_term_unicode)),
                )
            )
            .all()
        )
