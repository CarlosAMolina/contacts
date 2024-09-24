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
    def run_ask_input(self):
        pass

    @abstractmethod
    def run_search_value(self, search_value: str):
        pass


class IdSearch(_Search):
    def run_ask_input(self):
        print("Start search by ID")
        print("What ID do you want to see?")
        user_input = input()
        self.run_search_value(user_input)

    def run_search_value(self, search_value: str):
        print("Retrieving ID", search_value)
        # TODO


class TermSearch(_Search):
    def run_ask_input(self):
        print("Start search by term")
        print("Which term would you like to search?")
        search_term = input()
        self.run_search_value(search_term)

    def run_search_value(self, search_value: str):
        print("Searching value", search_value)
        body = self._get_body(search_value)
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
        users = self._get_users_sorted(users)
        summary_users_array = [self._get_summary_from_user(user) for user in users]
        return "\n".join(summary_users_array)

    # TODO implement this in GraphQL
    def _get_users_sorted(self, users: list[dict]) -> list[dict]:
        users_to_sort = [
            {
                "value_to_sort": self._get_value_as_ascii(user["name"]),
                **user,
            }
            for user in users
        ]
        users_sorted = sorted(users_to_sort, key=lambda k: k["value_to_sort"])
        for user in users_sorted:
            del user["value_to_sort"]
        return users_sorted

    @staticmethod
    def _get_value_as_ascii(value: str | None) -> str | None:
        if value is None:
            return None
        result = value.lower()
        replacements = {"á": "a", "é": "e", "í": "i", "ó": "o", "ú": "u"}
        for old_value, new_value in replacements.items():
            result = result.replace(old_value, new_value)
        return result

    def _get_summary_from_user(self, user: dict) -> str:
        if len(user["phones"]) == 0:
            summary_user = self._get_str_summary_without_phone_from_user(user)
        else:
            summary_phones_array = []
            phones_array_str = [self._get_str_summary_from_phone(phone) for phone in user["phones"]]
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
        # TODO modify the db to convert empty strings to nulls
        if user["surname"] is not None and len(user["surname"]) > 0:
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

    def _get_str_summary_from_phone(self, phone: dict) -> str:
        if len(phone["description"]) == 0:
            return str(phone["phone"])
        return "{phone} ({description})".format(phone=phone["phone"], description=phone["description"])

    def _get_str_nicknames_from_nicknames_dict(self, nicknames_dict: dict) -> str:
        return ", ".join(nickname["nickname"] for nickname in nicknames_dict)

    def _get_str_categories_from_cateogires_dict(self, categories_dict: dict) -> str:
        return ", ".join(category["category"]["category"] for category in categories_dict)


if __name__ == "__main__":
    TermSearch().run()
