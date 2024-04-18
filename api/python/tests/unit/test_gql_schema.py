import unittest


from src.gql.schema import schema


class TestSchemaQuery(unittest.TestCase):
    def test_resolve_email_all_fields_returns_expected_result(self):
        gql = """
        {
          email(emailId: 2) {
            id
            idUser
            email
            user {
              id
            }
          }
        }
        """
        schema_result = schema.execute(gql)
        result = schema_result.data["email"]
        expected_result = {
            "email": "jane_work@mail.com",
            "id": 2,
            "idUser": 2,
            "user": {"id": 2},
        }
        self.assertEqual(expected_result, result)

    def test_resolve_user_if_not_result(self):
        gql = """
        {
          user(userId: -5) {
            id
            name
            surname
          }
        }
        """
        schema_result = schema.execute(gql)
        result = schema_result.data["user"]
        self.assertIsNone(result)

    def test_resolve_user_all_fields_returns_expected_result(self):
        gql = """
        {
          user(userId: 2) {
            id
            name
            surname
            age
            emails {
              id
            }
          }
        }
        """
        schema_result = schema.execute(gql)
        result = schema_result.data["user"]
        expected_result = {
            "age": 30,
            "id": 2,
            "name": "Jane",
            "surname": None,
            "emails": [{"id": 1}, {"id": 2}],
        }
        self.assertEqual(expected_result, result)

    def test_resolve_users_by_min_age_returns_expected_result(self):
        gql = """
        {
          usersByMinAge(minAge: 25) {
            id
            name
          }
        }
        """
        schema_result = schema.execute(gql)
        result = schema_result.data["usersByMinAge"]
        expected_result = [{"id": 2, "name": "Jane"}]
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_value_without_results(self):
        gql = """
        {
          searchUser(searchTerm: \"asdf\") {
            id
            name
          }
        }
        """
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = []
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_term_in_name(self):
        gql = """
        {
          searchUser(searchTerm: \"a\") {
            id
            name
          }
        }
        """
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = [{"id": 2, "name": "Jane"}]
        self.assertEqual(expected_result, result)
