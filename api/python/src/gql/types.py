from graphene import Field
from graphene import List
from graphene import ObjectType
from graphene import Int
from graphene import String


class AddressObject(ObjectType):
    id = Int()
    id_user = Int()
    address = String()


class DiscordObject(ObjectType):
    id = Int()
    id_user = Int()
    user_name = String()
    discriminator = Int()
    alias = String()
    global_name = String()
    legacy_user_name = String()


class FacebookObject(ObjectType):
    id = Int()
    id_user = Int()
    url = String()


class EmailObject(ObjectType):
    id = Int()
    id_user = Int()
    email = String()
    user = Field(lambda: UserObject)


class UserObject(ObjectType):
    id = Int()
    name = String()
    surname = String()
    emails = List(lambda: EmailObject)
    addresses = List(lambda: AddressObject)
    discord = List(lambda: DiscordObject)
    facebook = List(lambda: FacebookObject)
