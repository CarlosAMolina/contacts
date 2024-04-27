"""
TODO refactor ..._unicode, for example using
https://docs.sqlalchemy.org/en/20/orm/extensions/hybrid.html#reusing-hybrid-properties-across-subclasses
"""

# TODO add users_categories

from sqlalchemy import Column
from sqlalchemy import ForeignKey
from sqlalchemy import func
from sqlalchemy import BigInteger
from sqlalchemy import Integer
from sqlalchemy import String
from sqlalchemy.orm import relationship
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.ext.hybrid import hybrid_property

from src.utils.unicode import ACCENT_TO_NO_ACCENT_MAP


Base = declarative_base()


class AddressModel(Base):
    __tablename__ = "addresses"

    id = Column(Integer, primary_key=True)
    id_user = Column(Integer, ForeignKey("users.id"), nullable=False)
    address = Column(String, nullable=False)
    user = relationship("UserModel", back_populates="addresses")

    @hybrid_property
    def address_unicode(self) -> Column[str]:
        return _get_column_unicode(self.address)


class CategoryModel(Base):
    __tablename__ = "categories"

    id = Column(Integer, primary_key=True)
    category = Column(String, nullable=False)
    user_category = relationship("UserCategoryModel", back_populates="category")

    @hybrid_property
    def category_unicode(self) -> Column[str]:
        return _get_column_unicode(self.category)


class DiscordModel(Base):
    __tablename__ = "discord"

    id = Column(Integer, primary_key=True)
    id_user = Column(Integer, ForeignKey("users.id"), nullable=False)
    user_name = Column(String, nullable=False)
    discriminator = Column(Integer)
    alias = Column(String)
    global_name = Column(String)
    legacy_user_name = Column(String)
    user = relationship("UserModel", back_populates="discord")

    @hybrid_property
    def user_name_unicode(self) -> Column[str]:
        return _get_column_unicode(self.user_name)

    @hybrid_property
    def alias_unicode(self) -> Column[str]:
        return _get_column_unicode(self.alias)

    @hybrid_property
    def global_name_unicode(self) -> Column[str]:
        return _get_column_unicode(self.global_name)

    @hybrid_property
    def legacy_user_name_unicode(self) -> Column[str]:
        return _get_column_unicode(self.legacy_user_name)


class EmailModel(Base):
    __tablename__ = "emails"

    id = Column(Integer, primary_key=True)
    id_user = Column(Integer, ForeignKey("users.id"), nullable=False)
    email = Column(String, nullable=False)
    user = relationship("UserModel", back_populates="emails")

    @hybrid_property
    def email_unicode(self) -> Column[str]:
        return _get_column_unicode(self.email)


class FacebookModel(Base):
    __tablename__ = "facebook"

    id = Column(Integer, primary_key=True)
    id_user = Column(Integer, ForeignKey("users.id"), nullable=False)
    url = Column(String, nullable=False)
    user = relationship("UserModel", back_populates="facebook")

    @hybrid_property
    def url_unicode(self) -> Column[str]:
        return _get_column_unicode(self.url)


class GitHubModel(Base):
    __tablename__ = "github"

    id = Column(Integer, primary_key=True)
    id_user = Column(Integer, ForeignKey("users.id"), nullable=False)
    url = Column(String, nullable=False)
    user = relationship("UserModel", back_populates="github")

    @hybrid_property
    def url_unicode(self) -> Column[str]:
        return _get_column_unicode(self.url)


class InstagramModel(Base):
    __tablename__ = "instagram"

    id = Column(Integer, primary_key=True)
    id_user = Column(Integer, ForeignKey("users.id"), nullable=False)
    handle = Column(String, nullable=False)
    user = relationship("UserModel", back_populates="instagram")

    @hybrid_property
    def handle_unicode(self) -> Column[str]:
        return _get_column_unicode(self.handle)


class LinkedinModel(Base):
    __tablename__ = "linkedin"

    id = Column(Integer, primary_key=True)
    id_user = Column(Integer, ForeignKey("users.id"), nullable=False)
    url = Column(String, nullable=False)
    user = relationship("UserModel", back_populates="linkedin")

    @hybrid_property
    def url_unicode(self) -> Column[str]:
        return _get_column_unicode(self.url)


class NicknameModel(Base):
    __tablename__ = "nicknames"

    id = Column(Integer, primary_key=True)
    id_user = Column(Integer, ForeignKey("users.id"), nullable=False)
    nickname = Column(String, nullable=False)
    user = relationship("UserModel", back_populates="nicknames")

    @hybrid_property
    def nickname_unicode(self) -> Column[str]:
        return _get_column_unicode(self.nickname)


class NoteModel(Base):
    __tablename__ = "notes"

    id = Column(Integer, primary_key=True)
    id_user = Column(Integer, ForeignKey("users.id"), nullable=False)
    note = Column(String, nullable=False)
    user = relationship("UserModel", back_populates="notes")

    @hybrid_property
    def note_unicode(self) -> Column[str]:
        return _get_column_unicode(self.note)


class PhoneModel(Base):
    __tablename__ = "phones"

    id = Column(Integer, primary_key=True)
    id_user = Column(Integer, ForeignKey("users.id"), nullable=False)
    phone = Column(BigInteger, nullable=False)
    description = Column(String)
    user = relationship("UserModel", back_populates="phones")

    @hybrid_property
    def description_unicode(self) -> Column[str]:
        return _get_column_unicode(self.description)


class TelegramModel(Base):
    __tablename__ = "telegram"

    id = Column(Integer, primary_key=True)
    id_user = Column(Integer, ForeignKey("users.id"), nullable=False)
    user_name = Column(String, nullable=False)
    user = relationship("UserModel", back_populates="telegram")

    @hybrid_property
    def user_name_unicode(self) -> Column[str]:
        return _get_column_unicode(self.user_name)


class TwitterModel(Base):
    __tablename__ = "twitter"

    id = Column(Integer, primary_key=True)
    id_user = Column(Integer, ForeignKey("users.id"), nullable=False)
    handle = Column(String, nullable=False)
    user = relationship("UserModel", back_populates="twitter")

    @hybrid_property
    def handle_unicode(self) -> Column[str]:
        return _get_column_unicode(self.handle)


class UserCategoryModel(Base):
    __tablename__ = "users_categories"

    id = Column(Integer, primary_key=True)
    id_user = Column(Integer, ForeignKey("users.id"), nullable=False)
    id_category = Column(Integer, ForeignKey("categories.id"), nullable=False)
    user = relationship("UserModel", back_populates="categories")
    # TODO use CategoryModel.category_unicode instead
    category = relationship("CategoryModel", back_populates="user_category")


class UrlModel(Base):
    __tablename__ = "urls"

    id = Column(Integer, primary_key=True)
    id_user = Column(Integer, ForeignKey("users.id"), nullable=False)
    url = Column(String, nullable=False)
    user = relationship("UserModel", back_populates="urls")

    @hybrid_property
    def url_unicode(self) -> Column[str]:
        return _get_column_unicode(self.url)


class UserModel(Base):
    """
    relationship, back_populates: allow query other class values.
    """

    __tablename__ = "users"

    id = Column(Integer, primary_key=True)
    name = Column(String, nullable=False)
    surname = Column(String)
    addresses = relationship("AddressModel", back_populates="user")
    categories = relationship("UserCategoryModel", back_populates="user")
    emails = relationship("EmailModel", back_populates="user")
    discord = relationship("DiscordModel", back_populates="user")
    facebook = relationship("FacebookModel", back_populates="user")
    github = relationship("GitHubModel", back_populates="user")
    linkedin = relationship("LinkedinModel", back_populates="user")
    instagram = relationship("InstagramModel", back_populates="user")
    nicknames = relationship("NicknameModel", back_populates="user")
    notes = relationship("NoteModel", back_populates="user")
    phones = relationship("PhoneModel", back_populates="user")
    telegram = relationship("TelegramModel", back_populates="user")
    twitter = relationship("TwitterModel", back_populates="user")
    urls = relationship("UrlModel", back_populates="user")
    wallapop = relationship("WallapopModel", back_populates="user")

    @hybrid_property
    def name_unicode(self) -> Column[str]:
        return _get_column_unicode(self.name)

    @hybrid_property
    def surname_unicode(self) -> Column[str]:
        return _get_column_unicode(self.surname)


# TODO note as nullable? change it in db too
class WallapopModel(Base):
    __tablename__ = "wallapop"

    id = Column(Integer, primary_key=True)
    id_user = Column(Integer, ForeignKey("users.id"), nullable=False)
    url = Column(String, nullable=False)
    note = Column(String, nullable=False)
    user = relationship("UserModel", back_populates="wallapop")

    @hybrid_property
    def url_unicode(self) -> Column[str]:
        return _get_column_unicode(self.url)

    @hybrid_property
    def note_unicode(self) -> Column[str]:
        return _get_column_unicode(self.note)


def _get_column_unicode(column: Column[str]) -> Column[str]:
    result = column
    for old, new in ACCENT_TO_NO_ACCENT_MAP.items():
        result = func.REPLACE(result, old, new)
    return result
