from search import DbAllData
import json


class Export:
    # Export required json by https://github.com/CarlosAMolina/contacts-js/tree/main
    def export_json(self):
        print("Exporting JSON")
        db_json = DbAllData().get_all_db()
        json_ = _Json(db_json)
        self._assert_all_ids_exist(json_.get_ids())
        json_to_export = json_.get_json_to_export()
        path_name_no_pretty = "/tmp/contacts.json"
        print("Exporting", path_name_no_pretty)
        with open(path_name_no_pretty, "w") as file:
            json.dump(json_to_export, file)
        path_name_pretty = "/tmp/contacts-pretty.json"
        print("Exporting", path_name_pretty)
        with open(path_name_pretty, "w") as file:
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
        users = self._get_users()
        return sorted([user["id"] for user in users])

    def get_json_to_export(self) -> dict:
        result = {"contacts": []}
        for user in self._get_users():
            user_dict = self._get_user_to_export(user)
            result["contacts"].append(user_dict)
        return result

    def _get_users(self) -> list[dict]:
        return self._json["data"]["usersWithTerm"]

    def _get_user_to_export(self, user: dict) -> dict:
        result = dict()
        if addresses := [address["address"] for address in user.get("address", [])]:
            result["addresses"] = addresses
        if categories := [category["category"]["category"] for category in user.get("categories", [])]:
            result["categories"] = categories
        result["id"] = user["id"]
        if emails := [email["email"] for email in user.get("emails", [])]:
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
        if nicknames := [nickname["nickname"] for nickname in user.get("nickname", [])]:
            result["nicknames"] = nicknames
        if note := user.get("note"):
            result["note"] = note
        if phones := [self._get_phone_to_export(phone) for phone in user.get("phones", [])]:
            result["phones"] = phones
        if surname := user.get("surname"):
            result["surname"] = surname
        if urls := [url["url"] for url in user.get("urls", [])]:
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
            if value := discord.get(key):
                result[key] = value
        return result

    def _get_wallapop_to_export(self, wallapop: dict) -> dict:
        result = {"url": wallapop["url"]}
        if note := wallapop.get("note"):
            result["note"] = note
        return result

    def _get_phone_to_export(self, phone: dict) -> dict:
        result = {"number": phone["phone"]}
        if description := phone.get("description"):
            result["description"] = description
        return result

    def _set_social_network(self, dict_: dict, key: str, value: object):
        if value:
            if "socialNetwork" not in dict_:
                dict_["socialNetwork"] = dict()
            dict_["socialNetwork"][key] = value
