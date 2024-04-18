"""
https://github.com/graphql-python/graphql-server/blob/master/docs/flask.md
"""

from graphene import Schema

from src.gql.query import Query


schema = Schema(query=Query)
