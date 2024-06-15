"""
https://github.com/graphql-python/graphql-server/blob/master/docs/flask.md
"""

import os

# TODO refactor move to files in src
from flask import Flask
from graphql_server.flask import GraphQLView

from src.db.database import db_session
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


@app.teardown_appcontext
def shutdown_session(exception=None):
    """
    https://flask.palletsprojects.com/en/3.0.x/patterns/sqlalchemy/
    """
    db_session.remove()


if __name__ == "__main__":
    print("GraphiQL URL: http://127.0.0.1:5000/graphql")
    # https://stackoverflow.com/a/30329547
    if os.getenv("IS_DOCKER"):
        # To detect IP used by the Docker container, when the container is running,
        # execute `docker network inspect bridge | grep IPv4Address`
        # https://stackoverflow.com/a/58138250
        app.run(host="172.17.0.2")
    else:
        app.run()
