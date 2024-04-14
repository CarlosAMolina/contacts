import unittest


from src.gql import main


class TestSchema(unittest.TestCase):
    def test_execute_schema_returns_expected_result(self):
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
