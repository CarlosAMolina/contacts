from abc import abstractmethod
from abc import ABC

import requests

from src.constants import GRAPHQL_URL
from src.constants import BODY_TERM_SEARCH


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

    def _get_body(self, search_term: str) -> str:
        return BODY_TERM_SEARCH.replace("{SEARCH_TERM}", search_term)


search_term = "que"
body = TermSearch()._get_body(search_term)
response = requests.post(url=GRAPHQL_URL, json={"query": body})
print("response status code: ", response.status_code)
if response.status_code == 200:
    print("response : ", response.json())
else:
    raise ValueError(f"GraphQL response: {response.content}")
