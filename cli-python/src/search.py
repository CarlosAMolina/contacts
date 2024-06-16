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
        # TODO


class TermSearch(Search):
    def run(self, search_term: str):
        print("Searching term", search_term)
        body = self._get_body(search_term)
        response_dict = self._get_dict_response(body)
        print(response_dict)

    def _get_body(self, search_term: str) -> str:
        return BODY_TERM_SEARCH.replace("{SEARCH_TERM}", search_term)

    def _get_dict_response(self, body: str) -> dict:
        response = requests.post(url=GRAPHQL_URL, json={"query": body})
        if response.status_code == 200:
            return response.json()
        else:
            raise ValueError(f"GraphQL response: {response.content}")


if __name__ == "__main__":
    search_term = "que"
    body = TermSearch()._get_body(search_term)
