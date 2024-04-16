import typing as tp

from graphene import Field
from graphene import List
from graphene import ObjectType
from graphene import Schema
from graphene import Int
from graphene import String

from src.db.data import users


class UserType(ObjectType):
    id = Int()
    name = String()
    surname = String()
    age = Int()


class Query(ObjectType):
    user = Field(UserType, user_id=Int())
    users_by_min_age = List(UserType, min_age=Int())

    @staticmethod
    def resolve_user(root, info, user_id=Int()) -> dict:
        matched_users = [user for user in users if user["id"] == user_id]
        return matched_users[0] if matched_users else None

    @staticmethod
    def resolve_users_by_min_age(root, info, min_age=Int()) -> tp.List[dict]:
        return [user for user in users if user["age"] >= min_age]


schema = Schema(query=Query)
