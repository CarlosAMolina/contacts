from graphene import Schema

from src.gql.query import Query


schema = Schema(query=Query)
