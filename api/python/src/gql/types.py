import typing as tp

from graphene import Field
from graphene import List
from graphene import ObjectType
from graphene import Int
from graphene import String


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

    @staticmethod
    def resolve_emails(root, info) -> tp.List[dict]:
        return root.emails
