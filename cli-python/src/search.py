from abc import abstractmethod
from abc import ABC

import requests


class Search(ABC):
    @abstractmethod
    def run(*args):
        pass


class IdSearch(Search):
    def run(self):
        print("Start displaying ID")
        print("What ID do you want to see?")
        user_input = input()
        print("Retrieving ID", user_input)


class TermSearch(Search):
    def run(self, user_input: str):
        print("Searching term", user_input)


GRAPHQL_URL = "http://127.0.0.1:5000/graphql"

body = """
{
  searchUser(searchTerm: "{SEARCH_TERM}") {
    id
    name
    surname
    categories {
      category {
        category
      }
    }
    nicknames {
      nickname
    }
    phones {
      phone
      description
    }
  }
}
"""


def get_body(search_term: str) -> str:
    return body.replace("{SEARCH_TERM}", search_term)


search_term = "que"
body = get_body(search_term)
response = requests.post(url=GRAPHQL_URL, json={"query": body})
print("response status code: ", response.status_code)
if response.status_code == 200:
    print("response : ", response.json())
else:
    raise ValueError(f"GraphQL response: {response.content}")
