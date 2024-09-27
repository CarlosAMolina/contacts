from abc import abstractmethod
from abc import ABC
from collections import namedtuple

import requests

try:
    from src import constants
except ModuleNotFoundError:
    import constants


class _Search(ABC):
    @abstractmethod
    def run_ask_input(self):
        pass

    @abstractmethod
    def run_search_value(self, search_value: str):
        pass

    def _get_dict_response(self, body: str) -> dict:
        response = requests.post(url=constants.GRAPHQL_URL, json={"query": body})
        if response.status_code != 200 or "errors" in response.json().keys():
            raise ValueError(f"GraphQL response: {response.content}")
        else:
            return response.json()


_SectionConfig = namedtuple("SectionConfig", "section_key summary_title value_key")


class IdSearch(_Search):
    def run_ask_input(self):
        print("Start search by ID")
        print("What ID do you want to see?")
        user_input = input()
        self.run_search_value(user_input)

    def run_search_value(self, search_value: str):
        print("Searching ID", search_value)
        print()
        body = self._get_body(search_value)
        response_dict = self._get_dict_response(body)
        summary = self._get_summary_from_response_dict(response_dict)
        print(summary)
        print()

    def _get_body(self, search_term: str) -> str:
        return constants.BODY_ID_SEARCH.replace("{SEARCH_TERM}", search_term)

    def _get_summary_from_response_dict(self, response_dict: dict) -> str:
        user = response_dict["data"]["user"]
        result = user["name"]
        # TODO check how no value is returned
        if user["surname"] is not None and len(user["surname"]) > 0:
            result += f" {user['surname']}"
        result = self._get_summary_add_section(_SectionConfig("addresses", "Addresses", "address"), result, user)
        if len(user["categories"]) > 0:
            result += "\nCategories:\n  "
            result += "\n  ".join(address["category"]["category"] for address in user["categories"])
        if len(user["discord"]) > 0:
            result += "\nDiscord:"
            result += "\n".join(self._get_discord_summary(discord) for discord in user["discord"])
        section_configs = [
            _SectionConfig("emails", "Emails", "email"),
            _SectionConfig("facebook", "Facebook", "url"),
            _SectionConfig("github", "GitHub", "url"),
            _SectionConfig("instagram", "Instagram", "handle"),
            _SectionConfig("linkedin", "LinkedIn", "url"),
            _SectionConfig("nicknames", "Motes", "nickname"),
            _SectionConfig("notes", "Notas", "note"),
        ]
        result = self._get_summary_apply_section_configs(section_configs, result, user)
        if len(user["phones"]) > 0:
            result += "\nTeléfonos:\n"
            result += "\n".join(self._get_phone_summary(phone) for phone in user["phones"])
        section_configs = [
            _SectionConfig("telegram", "Telegram", "userName"),
            _SectionConfig("tiktok", "TikTok", "userName"),
            _SectionConfig("twitter", "Twitter", "handle"),
            _SectionConfig("urls", "URLs", "url"),
        ]
        result = self._get_summary_apply_section_configs(section_configs, result, user)
        if len(user["wallapop"]) > 0:
            result += "\nWallapop:"
            result += "\n".join(self._get_wallapop_summary(wallapop) for wallapop in user["wallapop"])
        return result

    def _get_summary_apply_section_configs(self, configs: list[_SectionConfig], summary: str, user: dict) -> str:
        result = summary
        for config in configs:
            result = self._get_summary_add_section(config, result, user)
        return result

    def _get_summary_add_section(self, config: _SectionConfig, summary: str, user: dict) -> str:
        result = summary
        section_summary = self._get_summary_of_section(config, user)
        # TODO check how no value is returned
        if section_summary is not None:
            result += section_summary
        return result

    def _get_summary_of_section(self, config: _SectionConfig, user: dict) -> str | None:
        result = None
        if len(user[config.section_key]) > 0:
            result = f"\n{config.summary_title}:\n  "
            result += self._get_summary_for_list_of_dicts(user[config.section_key], config.value_key)
        return result

    def _get_summary_for_list_of_dicts(self, list_: list, key: str) -> str:
        return "\n  ".join(dict_[key] for dict_ in list_)

    def _get_discord_summary(self, discord: dict) -> str:
        result = f"\n  User name: {discord['userName']}"
        result += f"\n  Discriminator: {discord['discriminator']}"
        result += f"\n  Alias: {discord['alias']}"
        result += f"\n  Global name: {discord['globalName']}"
        result += f"\n  Legacy User Name: {discord['legacyUserName']}"
        return result

    def _get_phone_summary(self, phone: dict) -> str:
        result = f"  {phone['phone']}"
        # TODO check how no value is returned
        if phone["description"] is not None and len(phone["description"]) > 0:
            result += f" {phone['description']}"
        return result

    def _get_wallapop_summary(self, wallapop: dict) -> str:
        result = f"\n  Url: {wallapop['url']}"
        # TODO check how no value is returned
        if wallapop["note"] is not None and len(wallapop["note"]) > 0:
            result += f"\n  Note: {wallapop['note']}"
        return result


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
        return constants.BODY_TERM_SEARCH.replace("{SEARCH_TERM}", search_term)

    def _get_summary_from_response_dict(self, response_dict: dict) -> str:
        users = response_dict["data"]["usersWithTerm"]
        summary_users_array = [self._get_summary_from_user(user) for user in users]
        return "\n".join(summary_users_array)

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
