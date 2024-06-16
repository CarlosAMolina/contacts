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

    def test_get_summary_from_response_dict(self):
        expected_result = "TODO"
        response_dict = {"a": "foo"}
        result = m_search.TermSearch()._get_summary_from_response_dict(response_dict)
        self.assertEqual(expected_result, result)
