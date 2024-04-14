from graphene import Field
from graphene import ObjectType
from graphene import Schema
from graphene import Int
from graphene import String

from src.db.data import users


class UserType(ObjectType):
    id = Int()
    name = String()
    surname = String()


class Query(ObjectType):
    user = Field(UserType, user_id=Int())

    def resolve_user(self, info, user_id=Int()) -> dict:
        matched_users = [user for user in users if user["id"] == user_id]
        return matched_users[0] if matched_users else None


schema = Schema(query=Query)
