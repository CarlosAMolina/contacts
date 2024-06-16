from abc import abstractmethod
from abc import ABC

import requests

try:
    from src.constants import GRAPHQL_URL
    from src.constants import BODY_TERM_SEARCH
except ModuleNotFoundError:
    from constants import GRAPHQL_URL
    from constants import BODY_TERM_SEARCH


class _Search(ABC):
    @abstractmethod
    def run(*args):
        pass


class IdSearch(_Search):
    def run(self):
        print("Start displaying ID")
        print("What ID do you want to see?")
        user_input = input()
        print("Retrieving ID", user_input)
        # TODO


class TermSearch(_Search):
    def run(self, search_term: str):
        print("Searching term", search_term)
        body = self._get_body(search_term)
        response_dict = self._get_dict_response(body)
        summary = self._get_summary_from_response_dict(response_dict)
        print(summary)

    def _get_body(self, search_term: str) -> str:
        return BODY_TERM_SEARCH.replace("{SEARCH_TERM}", search_term)

    def _get_dict_response(self, body: str) -> dict:
        response = requests.post(url=GRAPHQL_URL, json={"query": body})
        if response.status_code == 200:
            return response.json()
        else:
            raise ValueError(f"GraphQL response: {response.content}")

    def _get_summary_from_response_dict(self, response_dict: dict) -> str:
        print(response_dict)  # TODO rm
        users = response_dict["data"]["searchUser"]
        result = ""
        for user in users:
            phones = user["phones"]
            if len(phones) == 0:
                result += self._get_str_summary_without_phone_from_user(user)
            else:
                phones_array_str = [
                    "{phone} ({description})".format(phone=phone["phone"], description=phone["description"])
                    for phone in phones
                ]
                for phone_str in phones_array_str:
                    summary_phone = "{phone_str}  {summary_no_phone}".format(
                        phone_str=phone_str,
                        summary_no_phone=self._get_str_summary_without_phone_from_user(user),
                    )
                    result += summary_phone
        return result

    def _get_str_summary_without_phone_from_user(self, user: dict) -> str:
        return "{name} {surname}. {nicknames_str}. {categories_str}. ID {id_}".format(
            name=user["name"],
            surname=user["surname"],
            nicknames_str=self._get_str_nicknames_from_nicknames_dict(user["nicknames"]),
            categories_str=self._get_str_categories_from_cateogires_dict(user["categories"]),
            id_=user["id"],
        )

    def _get_str_nicknames_from_nicknames_dict(self, nicknames_dict: dict) -> str:
        return ",".join(nickname["nickname"] for nickname in nicknames_dict)

    def _get_str_categories_from_cateogires_dict(self, categories_dict: dict) -> str:
        return ",".join(category["category"]["category"] for category in categories_dict)


if __name__ == "__main__":
    search_term = "que"
    body = TermSearch().run(search_term)
    print(body)
