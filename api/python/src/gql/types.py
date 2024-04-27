from graphene import BigInt
from graphene import Field
from graphene import List
from graphene import ObjectType
from graphene import Int
from graphene import String

# TODO? use `required`: https://docs.graphene-python.org/en/latest/types/scalars/


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


class UrlObject(ObjectType):
    id = Int()
    id_user = Int()
    url = String()


class FacebookObject(UrlObject):
    pass


class GitHubObject(UrlObject):
    pass


class InstagramObject(ObjectType):
    id = Int()
    id_user = Int()
    handle = String()


class LinkedinObject(UrlObject):
    pass


class NicknameObject(ObjectType):
    id = Int()
    id_user = Int()
    nickname = String()


class NoteObject(ObjectType):
    id = Int()
    id_user = Int()
    note = String()


class PhoneObject(ObjectType):
    id = Int()
    id_user = Int()
    phone = BigInt()
    description = String()


class UserObject(ObjectType):
    id = Int()
    addresses = List(lambda: AddressObject)
    discord = List(lambda: DiscordObject)
    emails = List(lambda: EmailObject)
    facebook = List(lambda: FacebookObject)
    github = List(lambda: GitHubObject)
    instagram = List(lambda: InstagramObject)
    linkedin = List(lambda: LinkedinObject)
    nicknames = List(lambda: NicknameObject)
    phones = List(lambda: PhoneObject)
    notes = List(lambda: NoteObject)
    name = String()
    surname = String()
