import typing as tp

from graphene import Field
from graphene import Int
from graphene import List
from graphene import ObjectType
from graphene import String
from sqlalchemy import and_
from sqlalchemy import or_

from src.db.database import db_session
from src.db import models
from src.gql.types import EmailObject
from src.gql.types import UserObject
from src.utils.unicode import get_string_unicode


class Query(ObjectType):
    email = Field(EmailObject, email_id=Int())
    user = Field(UserObject, user_id=Int())
    users_with_term = List(
        UserObject, search_term=String(), sort=String(), sort_by=String()
    )  # List field for search results

    @staticmethod
    def resolve_email(root, info, email_id=Int()) -> tp.Optional[models.EmailModel]:
        return db_session.query(models.EmailModel).filter(models.EmailModel.id == email_id).first()

    @staticmethod
    def resolve_user(root, info, user_id=Int()) -> tp.Optional[models.UserModel]:
        return db_session.query(models.UserModel).filter(models.UserModel.id == user_id).first()

    @staticmethod
    def resolve_users_with_term(
        root, info, search_term=String(), sort="ASCENDING", sort_by="name_unicode"
    ) -> tp.Optional[tp.List[models.UserModel]]:
        """
        https://stackoverflow.com/questions/8561470/sqlalchemy-filtering-by-relationship-attribute
        """
        search_term_unicode = get_string_unicode(search_term)
        sort_function_config = {"ASCENDING": "asc", "DESCENDING": "desc"}
        order_by = getattr(getattr(models.UserModel, sort_by), sort_function_config[sort])()
        return (
            db_session.query(models.UserModel)
            .filter(
                or_(
                    models.UserModel.addresses.any(models.AddressModel.address_unicode.contains(search_term_unicode)),
                    models.UserModel.categories.any(
                        and_(
                            models.CategoryModel.category_unicode.contains(search_term_unicode),
                            models.CategoryModel.id == models.UserCategoryModel.id_category,
                        ),
                    ),
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
                    models.UserModel.tiktok.any(models.TiktokModel.user_name_unicode.contains(search_term_unicode)),
                    models.UserModel.twitter.any(models.TwitterModel.handle_unicode.contains(search_term_unicode)),
                    models.UserModel.urls.any(models.UrlModel.url_unicode.contains(search_term_unicode)),
                    models.UserModel.wallapop.any(models.WallapopModel.url_unicode.contains(search_term_unicode)),
                    models.UserModel.wallapop.any(models.WallapopModel.note_unicode.contains(search_term_unicode)),
                    models.UserModel.name_unicode.contains(search_term_unicode),
                    models.UserModel.surname_unicode.contains(search_term_unicode),
                )
            )
            .order_by(order_by)
            .all()
        )
