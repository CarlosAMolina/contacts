import unittest

from src import search as m_search


class TestTermSearch(unittest.TestCase):
    def test_get_body(self):
        expected_result = """
{
  searchUser(searchTerm: "que") {
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
                "searchUser": [
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

    def test_get_summary_from_response_dict_if_user_with_all_values_only_one_value_for_each_and_empty_phone(self):
        response_dict = {
            "data": {
                "searchUser": [
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

    # TODO
    def _test_get_summary_from_response_dict_if_multiple_users(self):
        response_dict = {
            "data": {
                "searchUser": [
                    {
                        "id": 2,
                        "name": "John",
                        "surname": "Doe",
                        "categories": [{"category": {"category": "Family"}}, {"category": {"category": "Work"}}],
                        "nicknames": [{"nickname": "j"}, {"nickname": "j2"}],
                        "phones": [
                            {"phone": 666666661, "description": "personal"},
                            {"phone": 666666662, "description": "work"},
                        ],
                    },
                    {
                        "id": 3,
                        "name": "Jane",
                        "surname": "Doe",
                        "categories": [{"category": {"category": "Friends"}}],
                        "nicknames": [{"nickname": "ja"}],
                        "phones": [{"phone": 666666661, "description": "personal"}],
                    },
                ]
            }
        }
        result = m_search.TermSearch()._get_summary_from_response_dict(response_dict)
        expected_result = "John"
        self.assertEqual(expected_result, result)
