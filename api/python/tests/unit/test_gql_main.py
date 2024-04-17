import unittest


from src.gql import main


class TestSchema(unittest.TestCase):
    def test_resolve_user_returns_expected_result(self):
        gql = """
        {
          user(userId: 2) {
            id
            name
            surname
          }
        }
        """
        schema_result = main.schema.execute(gql)
        result = schema_result.data["user"]
        expected_result = {"id": 2, "name": "Jane", "surname": None}
        self.assertEqual(expected_result, result)

    # TODO def test_resolve_user_with_emails_returns_expected_result(self):
    # TODO     gql = """
    # TODO     {
    # TODO       user(userId: 2) {
    # TODO         id
    # TODO         name
    # TODO         surname
    # TODO         emails
    # TODO       }
    # TODO     }
    # TODO     """
    # TODO     schema_result = main.schema.execute(gql)
    # TODO     result = schema_result.data["user"]
    # TODO     expected_result = {"id": 2, "name": "Jane", "surname": None, "emails": "TODO"}
    # TODO     self.assertEqual(expected_result, result)

    def test_resolve_users_min_age_returns_expected_result(self):
        gql = """
        {
          usersByMinAge(minAge: 25) {
            id
            name
            surname
            age
          }
        }
        """
        schema_result = main.schema.execute(gql)
        result = schema_result.data["usersByMinAge"]
        expected_result = [{"age": 30, "id": 2, "name": "Jane", "surname": None}]
        self.assertEqual(expected_result, result)
