from search import AllSearch
import json


class Export:
    def export_json(self):
        print("Exporting JSON")
        db_json = AllSearch().get_all_db()
        json_ = _Json(db_json)
        self._assert_all_ids_exist(json_.get_ids())
        json_to_export = json_.get_json_to_export()
        with open("/tmp/contacts.json", "w") as file:
            json.dump(json_to_export, file)
        with open("/tmp/contacts-pretty.json", "w") as file:
            json.dump(json_to_export, file, indent=2)

    def _assert_all_ids_exist(self, ids: list[int]):
        expected_value = 1
        for id_ in ids:
            if not id_ == expected_value:
                raise ValueError(f"Lost ID: {id_}")
            expected_value += 1


class _Json:
    def __init__(self, json_: dict):
        self._json = json_

    def get_ids(self) -> list[int]:
        return sorted([user["id"] for user in self._get_users()])

    def get_json_to_export(self) -> dict:
        result = dict()
        result["data"] = {"allContacts": []}
        for user in self._get_users():
            user_dict = self._get_user_to_export(user)
            result["data"]["allContacts"].append(user_dict)
        return result

    def _get_users(self) -> list[dict]:
        return self._json["data"]["usersWithTerm"]

    def _get_user_to_export(self, user: dict) -> dict:
        result = dict()
        addresses = [address["address"] for address in user.get("address", [])]
        if addresses:
            result["addresses"] = addresses
        categories = [category["category"]["category"] for category in user.get("categories", [])]
        if categories:
            result["categories"] = categories
        result["id"] = user["id"]
        emails = [email["email"] for email in user.get("emails", [])]
        if emails:
            result["emails"] = emails
        discord_accounts = [self._get_discord_to_export(discord) for discord in user.get("discord", [])]
        self._set_social_network(result, "discordAccounts", discord_accounts)
        facebook_accounts = [facebook["url"] for facebook in user.get("facebook", [])]
        self._set_social_network(result, "facebookAccounts", facebook_accounts)
        github_accounts = [github["url"] for github in user.get("github", [])]
        self._set_social_network(result, "githubAccounts", github_accounts)
        instagram_accounts = [instagram["handle"] for instagram in user.get("instagram", [])]
        self._set_social_network(result, "instagramAccounts", instagram_accounts)
        linkedin_accounts = [linkedin["url"] for linkedin in user.get("linkedin", [])]
        self._set_social_network(result, "linkedinAccounts", linkedin_accounts)
        telegram_accounts = [telegram["userName"] for telegram in user.get("telegram", [])]
        self._set_social_network(result, "telegramAccounts", telegram_accounts)
        tiktok_accounts = [tiktok["userName"] for tiktok in user.get("tiktok", [])]
        self._set_social_network(result, "tiktokAccounts", tiktok_accounts)
        twitter_accounts = [twitter["handle"] for twitter in user.get("twitter", [])]
        self._set_social_network(result, "twitterAccounts", twitter_accounts)
        wallapop_accounts = [self._get_wallapop_to_export(wallapop) for wallapop in user.get("wallapop", [])]
        self._set_social_network(result, "wallapopAccounts", wallapop_accounts)
        result["name"] = user["name"]
        nicknames = [nickname["nickname"] for nickname in user.get("nickname", [])]
        if nicknames:
            result["nicknames"] = nicknames
        if user.get("note"):
            result["note"] = user["note"]
        phones = [self._get_phone_to_export(phone) for phone in user.get("phones", [])]
        if phones:
            result["phones"] = phones
        if user.get("surname"):
            result["surname"] = user["surname"]
        urls = [url["url"] for url in user.get("urls", [])]
        if urls:
            result["urls"] = urls
        return result

    def _get_discord_to_export(self, discord: dict) -> dict:
        result = dict()
        for key in [
            "alias",
            "discriminator",
            "globalName",
            "legacyUserName",
            "userName",
        ]:
            if discord.get(key):
                result[key] = discord[key]
        return result

    def _get_wallapop_to_export(self, wallapop: dict) -> dict:
        result = {"url": wallapop["url"]}
        if wallapop.get("note"):
            result["note"] = wallapop["note"]
        return result

    def _get_phone_to_export(self, phone: dict) -> dict:
        result = {"phone": phone["phone"]}
        if phone.get("description"):
            result["description"] = phone["description"]
        return result

    def _set_social_network(self, dict_: dict, key: str, value: object):
        if value:
            if "socialNetwork" not in dict_:
                dict_["socialNetwork"] = dict()
            dict_["socialNetwork"][key] = value
