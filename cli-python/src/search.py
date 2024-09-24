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
    def run(self):
        pass


class IdSearch(_Search):
    def run(self):
        print("Start search by ID")
        print("What ID do you want to see?")
        user_input = input()
        print("Retrieving ID", user_input)
        # TODO


class TermSearch(_Search):
    def run(self):
        print("Start search by term")
        print("Which term would you like to search?")
        search_term = input()
        print("Searching term", search_term)
        body = self._get_body(search_term)
        response_dict = self._get_dict_response(body)
        summary = self._get_summary_from_response_dict(response_dict)
        print(summary)
        print()

    def _get_body(self, search_term: str) -> str:
        return BODY_TERM_SEARCH.replace("{SEARCH_TERM}", search_term)

    def _get_dict_response(self, body: str) -> dict:
        response = requests.post(url=GRAPHQL_URL, json={"query": body})
        if response.status_code != 200 or "errors" in response.json().keys():
            raise ValueError(f"GraphQL response: {response.content}")
        else:
            return response.json()

    def _get_summary_from_response_dict(self, response_dict: dict) -> str:
        users = response_dict["data"]["searchUser"]
        summary_users_array = []
        for user in users:
            summary_user = self._get_summary_from_user(user)
            summary_users_array.append(summary_user)
        return "\n".join(summary_users_array)

    def _get_summary_from_user(self, user: dict) -> str:
        if len(user["phones"]) == 0:
            summary_user = self._get_str_summary_without_phone_from_user(user)
        else:
            summary_phones_array = []
            phones_array_str = [
                "{phone} ({description})".format(phone=phone["phone"], description=phone["description"])
                for phone in user["phones"]
            ]
            for phone_str in phones_array_str:
                summary_phone = "{phone_str}  {summary_no_phone}".format(
                    phone_str=phone_str,
                    summary_no_phone=self._get_str_summary_without_phone_from_user(user),
                )
                summary_phones_array.append(summary_phone)
            summary_user = "\n".join(summary_phones_array)
        return summary_user

    def _get_str_summary_without_phone_from_user(self, user: dict) -> str:
        result = ""
        if user["name"] is not None:
            result += "{name}".format(name=user["name"])
        if user["surname"] is not None:
            if len(result) > 0:
                result += " "
            result += "{surname}".format(surname=user["surname"])
        nicknames_str = self._get_str_nicknames_from_nicknames_dict(user["nicknames"])
        if len(nicknames_str) > 0:
            if len(result) > 0:
                result += ". "
            result += f"{nicknames_str}"
        categories_str = self._get_str_categories_from_cateogires_dict(user["categories"])
        if len(categories_str) > 0:
            result += f". {categories_str}"
        return "{result}. ID {id_}".format(
            result=result,
            id_=user["id"],
        )

    def _get_str_nicknames_from_nicknames_dict(self, nicknames_dict: dict) -> str:
        return ", ".join(nickname["nickname"] for nickname in nicknames_dict)

    def _get_str_categories_from_cateogires_dict(self, categories_dict: dict) -> str:
        return ", ".join(category["category"]["category"] for category in categories_dict)


if __name__ == "__main__":
    TermSearch().run()
