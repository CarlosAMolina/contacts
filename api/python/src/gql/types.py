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


class EmailObject(ObjectType):
    id = Int()
    id_user = Int()
    email = String()
    user = Field(lambda: UserObject)


class FacebookObject(ObjectType):
    id = Int()
    id_user = Int()
    url = String()


class UserObject(ObjectType):
    id = Int()
    addresses = List(lambda: AddressObject)
    discord = List(lambda: DiscordObject)
    emails = List(lambda: EmailObject)
    facebook = List(lambda: FacebookObject)
    name = String()
    surname = String()
