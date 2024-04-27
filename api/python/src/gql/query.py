import typing as tp

from graphene import Field
from graphene import Int
from graphene import List
from graphene import ObjectType
from graphene import String
from sqlalchemy import or_

from src.db.database import db_session
from src.db import models
from src.gql.types import EmailObject
from src.gql.types import UserObject
from src.utils.unicode import get_string_unicode


class Query(ObjectType):
    email = Field(EmailObject, email_id=Int())
    user = Field(UserObject, user_id=Int())
    search_user = List(UserObject, search_term=String())  # List field for search results

    @staticmethod
    def resolve_email(root, info, email_id=Int()) -> tp.Optional[models.EmailModel]:
        return db_session.query(models.EmailModel).filter(models.EmailModel.id == email_id).first()

    @staticmethod
    def resolve_user(root, info, user_id=Int()) -> tp.Optional[models.UserModel]:
        return db_session.query(models.UserModel).filter(models.UserModel.id == user_id).first()

    @staticmethod
    def resolve_search_user(root, info, search_term=String()) -> tp.Optional[tp.List[models.UserModel]]:
        """
        https://stackoverflow.com/questions/8561470/sqlalchemy-filtering-by-relationship-attribute
        """
        search_term_unicode = get_string_unicode(search_term)
        return (
            db_session.query(models.UserModel)
            .filter(
                or_(
                    models.UserModel.addresses.any(models.AddressModel.address_unicode.contains(search_term_unicode)),
                    models.UserModel.discord.any(models.DiscordModel.alias_unicode.contains(search_term_unicode)),
                    models.UserModel.discord.any(models.DiscordModel.discriminator.contains(search_term_unicode)),
                    models.UserModel.discord.any(models.DiscordModel.global_name_unicode.contains(search_term_unicode)),
                    models.UserModel.discord.any(
                        models.DiscordModel.legacy_user_name_unicode.contains(search_term_unicode)
                    ),
                    models.UserModel.discord.any(models.DiscordModel.user_name_unicode.contains(search_term_unicode)),
                    models.UserModel.emails.any(models.EmailModel.email_unicode.contains(search_term_unicode)),
                    models.UserModel.facebook.any(models.FacebookModel.url_unicode.contains(search_term_unicode)),
                    models.UserModel.github.any(models.GitHubModel.url_unicode.contains(search_term_unicode)),
                    models.UserModel.instagram.any(models.InstagramModel.handle_unicode.contains(search_term_unicode)),
                    models.UserModel.linkedin.any(models.LinkedinModel.url_unicode.contains(search_term_unicode)),
                    models.UserModel.nicknames.any(models.NicknameModel.nickname_unicode.contains(search_term_unicode)),
                    models.UserModel.notes.any(models.NoteModel.note_unicode.contains(search_term_unicode)),
                    models.UserModel.phones.any(models.PhoneModel.phone.contains(search_term_unicode)),
                    models.UserModel.phones.any(models.PhoneModel.description_unicode.contains(search_term_unicode)),
                    models.UserModel.telegram.any(models.TelegramModel.user_name_unicode.contains(search_term_unicode)),
                    models.UserModel.twitter.any(models.TwitterModel.handle_unicode.contains(search_term_unicode)),
                    models.UserModel.urls.any(models.UrlModel.url_unicode.contains(search_term_unicode)),
                    models.UserModel.name_unicode.contains(search_term_unicode),
                    models.UserModel.surname_unicode.contains(search_term_unicode),
                )
            )
            .all()
        )
