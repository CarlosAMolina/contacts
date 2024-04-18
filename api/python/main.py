"""
https://github.com/graphql-python/graphql-server/blob/master/docs/flask.md
"""

# TODO refactor move to files in src
from flask import Flask
from graphql_server.flask import GraphQLView

from src.gql.schema import schema

app = Flask(__name__)

app.add_url_rule(
    "/graphql",
    view_func=GraphQLView.as_view(
        "graphql",
        schema=schema,
        graphiql=True,
    ),
)

if __name__ == "__main__":
    print("GraphiQL URL: http://127.0.0.1:5000/graphql")
    app.run()