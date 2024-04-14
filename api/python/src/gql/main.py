from graphene import ObjectType
from graphene import Schema
from graphene import String


class Query(ObjectType):
    hello = String(name=String(default_value="world"))

    def resolve_hello(self, info, name: str) -> str:
        return f"Hello {name}"


schema = Schema(query=Query)

gql = """
{
  hello(name: "John")
}
"""

if __name__ == "__main__":
    result = schema.execute(gql)
    print(result)
