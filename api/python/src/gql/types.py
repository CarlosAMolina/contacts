import typing as tp

from graphene import Field
from graphene import List
from graphene import ObjectType
from graphene import Int
from graphene import String


class AddressObject(ObjectType):
    id = Int()
    id_user = Int()
    address = String()
    user = Field(lambda: UserObject)

    @staticmethod
    def resolve_user(root, info) -> tp.Optional[dict]:
        return root.user


class DiscordObject(ObjectType):
    id = Int()
    id_user = Int()
    user_name = String()
    discriminator = Int()
    alias = String()
    global_name = String()
    legacy_user_name = String()
    user = Field(lambda: UserObject)

    @staticmethod
    def resolve_user(root, info) -> tp.Optional[dict]:
        return root.user


class EmailObject(ObjectType):
    id = Int()
    id_user = Int()
    email = String()
    user = Field(lambda: UserObject)

    @staticmethod
    def resolve_user(root, info) -> tp.Optional[dict]:
        return root.user


class UserObject(ObjectType):
    id = Int()
    name = String()
    surname = String()
    emails = List(lambda: EmailObject)
    addresses = List(lambda: AddressObject)
    discord = List(lambda: DiscordObject)

    @staticmethod
    def resolve_emails(root, info) -> tp.List[dict]:
        return root.emails
