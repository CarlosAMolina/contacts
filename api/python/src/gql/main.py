import typing as tp

from graphene import Field
from graphene import List
from graphene import ObjectType
from graphene import Schema
from graphene import Int
from graphene import String

from src.db import data


class UserObject(ObjectType):
    id = Int()
    name = String()
    surname = String()
    age = Int()
    emails = List(lambda: EmailObject)

    @staticmethod
    def resolve_emails(root, info) -> tp.List[dict]:
        return [email for email in data.emails if email["id_user"] == root["id"]]


class EmailObject(ObjectType):
    id = Int()
    id_user = Int()
    email = String()
    user = Field(lambda: UserObject)

    @staticmethod
    def resolve_user(root, info) -> tp.Optional[dict]:
        matched_users = [user for user in data.users if user["id"] == root["id_user"]]
        return matched_users[0] if matched_users else None


class Query(ObjectType):
    user = Field(UserObject, user_id=Int())
    users_by_min_age = List(UserObject, min_age=Int())

    @staticmethod
    def resolve_user(root, info, user_id=Int()) -> dict:
        matched_users = [user for user in data.users if user["id"] == user_id]
        return matched_users[0] if matched_users else None

    @staticmethod
    def resolve_users_by_min_age(root, info, min_age=Int()) -> tp.List[dict]:
        return [user for user in data.users if user["age"] >= min_age]


schema = Schema(query=Query)
