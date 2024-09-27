import unittest
from pathlib import Path

from src import search as m_search

current_path = Path(__file__).parent.resolve()


class TestIdSearch(unittest.TestCase):
    def test_get_summary_from_response_dict_returns_expected_value_if_multiple_values_for_all_fields(self):
        response_dict = {
            "data": {
                "user": {
                    "name": "John",
                    "surname": "Doe Deo",
                    "addresses": [{"address": "c/ 1"}, {"address": "c/ 2"}],
                    "categories": [{"category": {"category": "university"}}, {"category": {"category": "job"}}],
                    "discord": [
                        {
                            "userName": "foo",
                            "discriminator": 3,
                            "alias": "bar",
                            "globalName": "baz",
                            "legacyUserName": "qux",
                        },
                        {
                            "userName": "foo2",
                            "discriminator": 5,
                            "alias": "bar2",
                            "globalName": "baz2",
                            "legacyUserName": "qux2",
                        },
                    ],
                    "emails": [{"email": "a@a.com"}, {"email": "b@b.com"}],
                    "facebook": [{"url": "https://www.facebook.com/a/"}, {"url": "https://www.facebook.com/b/"}],
                    "github": [{"url": "https://github.com/a/"}, {"url": "https://github.com/b/"}],
                    "instagram": [{"handle": "foo"}, {"handle": "bar"}],
                    "linkedin": [{"url": "https://linkedin.com/a/"}, {"url": "https://linkedin.com/b/"}],
                    "nicknames": [{"nickname": "foo"}, {"nickname": "bar"}],
                    "notes": [{"note": "asdf 1"}, {"note": "asdf 2"}],
                    "phones": [
                        {"phone": 666666661, "description": "job"},
                        {"phone": 666666662, "description": "personal"},
                    ],
                    "telegram": [{"userName": "foo"}, {"userName": "bar"}],
                    "tiktok": [{"userName": "foo"}, {"userName": "bar"}],
                    "twitter": [{"handle": "foo"}, {"handle": "bar"}],
                    "urls": [{"url": "url.com"}, {"url": "url2.com"}],
                    "wallapop": [
                        {"url": "https://wallapop.com/app/user/foo", "note": "user url"},
                        {"url": "https://wallapop.com/app/item/foo", "note": "car buy"},
                    ],
                }
            }
        }
        result = m_search.IdSearch()._get_summary_from_response_dict(response_dict)
        with open(current_path.joinpath("result-id-search.txt"), "r") as f:
            expected_result = f.read().rstrip()
        self.assertEqual(expected_result, result)


class TestTermSearch(unittest.TestCase):
    def test_get_body(self):
        expected_result = """
{
  usersWithTerm(searchTerm: "que") {
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
        result = m_search.TermSearch()._get_body("que")
        self.assertEqual(expected_result, result)

    def test_get_summary_from_response_dict_if_user_with_all_values_only_one_value_for_each(self):
        response_dict = {
            "data": {
                "usersWithTerm": [
                    {
                        "id": 1,
                        "name": "John",
                        "surname": "Doe",
                        "categories": [{"category": {"category": "Family"}}],
                        "nicknames": [{"nickname": "j"}],
                        "phones": [{"phone": 666666661, "description": "personal"}],
                    }
                ]
            }
        }
        result = m_search.TermSearch()._get_summary_from_response_dict(response_dict)
        expected_result = "666666661 (personal)  John Doe. j. Family. ID 1"
        self.assertEqual(expected_result, result)

    def test_get_summary_from_response_dict_if_user_with_all_values_multiple_values_for_each(self):
        response_dict = {
            "data": {
                "usersWithTerm": [
                    {
                        "id": 1,
                        "name": "John",
                        "surname": "Doe",
                        "categories": [{"category": {"category": "Family"}}, {"category": {"category": "Work"}}],
                        "nicknames": [{"nickname": "j"}, {"nickname": "j2"}],
                        "phones": [
                            {"phone": 666666661, "description": "personal"},
                            {"phone": 666666662, "description": "work"},
                        ],
                    }
                ]
            }
        }

        result = m_search.TermSearch()._get_summary_from_response_dict(response_dict)
        expected_result = (
            "666666661 (personal)  John Doe. j, j2. Family, Work. ID 1"
            "\n666666662 (work)  John Doe. j, j2. Family, Work. ID 1"
        )
        self.assertEqual(expected_result, result)

    # TODO verify no name is returned as None
    def test_get_summary_from_response_dict_if_user_with_all_values_only_one_value_for_each_and_empty_name(self):
        response_dict = {
            "data": {
                "usersWithTerm": [
                    {
                        "id": 1,
                        "name": None,
                        "surname": "Doe",
                        "categories": [{"category": {"category": "Family"}}],
                        "nicknames": [{"nickname": "j"}],
                        "phones": [{"phone": 666666661, "description": "personal"}],
                    }
                ]
            }
        }
        result = m_search.TermSearch()._get_summary_from_response_dict(response_dict)
        expected_result = "666666661 (personal)  Doe. j. Family. ID 1"
        self.assertEqual(expected_result, result)

    # TODO verify no surname is returned as empty string
    def test_get_summary_from_response_dict_if_user_with_all_values_only_one_value_for_each_and_empty_surname(self):
        response_dict = {
            "data": {
                "usersWithTerm": [
                    {
                        "id": 1,
                        "name": "John",
                        "surname": "",
                        "categories": [{"category": {"category": "Family"}}],
                        "nicknames": [{"nickname": "j"}],
                        "phones": [{"phone": 666666661, "description": "personal"}],
                    }
                ]
            }
        }
        result = m_search.TermSearch()._get_summary_from_response_dict(response_dict)
        expected_result = "666666661 (personal)  John. j. Family. ID 1"
        self.assertEqual(expected_result, result)

    # TODO verify no surname is returned as None
    def test_get_summary_from_response_dict_if_user_with_all_values_only_one_value_for_each_and_empty_name_surname(
        self,
    ):
        response_dict = {
            "data": {
                "usersWithTerm": [
                    {
                        "id": 1,
                        "name": None,
                        "surname": None,
                        "categories": [{"category": {"category": "Family"}}],
                        "nicknames": [{"nickname": "j"}],
                        "phones": [{"phone": 666666661, "description": "personal"}],
                    }
                ]
            }
        }
        result = m_search.TermSearch()._get_summary_from_response_dict(response_dict)
        expected_result = "666666661 (personal)  j. Family. ID 1"
        self.assertEqual(expected_result, result)

    def test_get_summary_from_response_dict_if_user_with_all_values_only_one_value_for_each_and_empty_categories(self):
        response_dict = {
            "data": {
                "usersWithTerm": [
                    {
                        "id": 1,
                        "name": "John",
                        "surname": "Doe",
                        "categories": [],
                        "nicknames": [{"nickname": "j"}],
                        "phones": [{"phone": 666666661, "description": "personal"}],
                    }
                ]
            }
        }
        result = m_search.TermSearch()._get_summary_from_response_dict(response_dict)
        expected_result = "666666661 (personal)  John Doe. j. ID 1"
        self.assertEqual(expected_result, result)

    def test_get_summary_from_response_dict_if_user_with_all_values_only_one_value_for_each_and_empty_nickname(self):
        response_dict = {
            "data": {
                "usersWithTerm": [
                    {
                        "id": 1,
                        "name": "John",
                        "surname": "Doe",
                        "categories": [{"category": {"category": "Family"}}],
                        "nicknames": [],
                        "phones": [{"phone": 666666661, "description": "personal"}],
                    }
                ]
            }
        }
        result = m_search.TermSearch()._get_summary_from_response_dict(response_dict)
        expected_result = "666666661 (personal)  John Doe. Family. ID 1"
        self.assertEqual(expected_result, result)

    def test_get_summary_from_response_dict_if_user_with_all_values_only_one_value_for_each_and_empty_phone(self):
        response_dict = {
            "data": {
                "usersWithTerm": [
                    {
                        "id": 1,
                        "name": "John",
                        "surname": "Doe",
                        "categories": [{"category": {"category": "Family"}}],
                        "nicknames": [{"nickname": "j"}],
                        "phones": [],
                    }
                ]
            }
        }
        result = m_search.TermSearch()._get_summary_from_response_dict(response_dict)
        expected_result = "John Doe. j. Family. ID 1"
        self.assertEqual(expected_result, result)

    # TODO veriy empty phone description from API is empty string
    def test_get_summary_from_response_dict_if_user_with_all_values_only_one_value_for_each_and_empty_phone_description(
        self,
    ):
        response_dict = {
            "data": {
                "usersWithTerm": [
                    {
                        "id": 1,
                        "name": "John",
                        "surname": "Doe",
                        "categories": [{"category": {"category": "Family"}}],
                        "nicknames": [{"nickname": "j"}],
                        "phones": [{"phone": 666666661, "description": ""}],
                    }
                ]
            }
        }
        result = m_search.TermSearch()._get_summary_from_response_dict(response_dict)
        expected_result = "666666661  John Doe. j. Family. ID 1"
        self.assertEqual(expected_result, result)

    def test_get_summary_from_response_dict_if_multiple_users(self):
        # Note. Sort is done in the API.
        response_dict = {
            "data": {
                "usersWithTerm": [
                    {
                        "id": 1,
                        "name": "John",
                        "surname": "Doe",
                        "categories": [{"category": {"category": "Family"}}],
                        "nicknames": [{"nickname": "j"}],
                        "phones": [{"phone": 666666661, "description": "personal"}],
                    },
                    {
                        "id": 2,
                        "name": "Jane",
                        "surname": "Do",
                        "categories": [{"category": {"category": "Friends"}}],
                        "nicknames": [{"nickname": "ja"}],
                        "phones": [{"phone": 666666662, "description": "work"}],
                    },
                ]
            }
        }
        result = m_search.TermSearch()._get_summary_from_response_dict(response_dict)
        expected_result = (
            "666666661 (personal)  John Doe. j. Family. ID 1\n666666662 (work)  Jane Do. ja. Friends. ID 2"
        )
        self.assertEqual(expected_result, result)
