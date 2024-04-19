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
            "email": "unique_mail_value_b@mail.com",
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
            emails {
              id
              email
            }
            addresses {
              id
              address
            }
          }
        }
        """
        schema_result = schema.execute(gql)
        result = schema_result.data["user"]
        expected_result = {
            "id": 2,
            "name": "unique name value",
            "surname": "unique surname value",
            "emails": [
                {
                    "id": 1,
                    "email": "unique_mail_value@mail.com",
                },
                {
                    "id": 2,
                    "email": "unique_mail_value_b@mail.com",
                },
            ],
            "addresses": [
                {
                    "id": 1,
                    "address": "unique address value",
                },
                {
                    "id": 2,
                    "address": "C/ Camión",
                },
            ],
        }
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_value_without_results(self):
        gql = self._get_graphql_search_user_query("asdf")
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = []
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_term_in_name(self):
        gql = self._get_graphql_search_user_query("unique name val")
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = [{"id": 2}]
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_term_in_name_is_case_insensitive(self):
        gql = self._get_graphql_search_user_query("UNIQUE NAME VAL")
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = [{"id": 2}]
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_term_in_surname(self):
        gql = self._get_graphql_search_user_query("unique surname val")
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = [{"id": 2}]
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_term_in_emails(self):
        gql = self._get_graphql_search_user_query("unique_mail_value@m")
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = [{"id": 2}]
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_term_in_addresses(self):
        gql = self._get_graphql_search_user_query("unique address val")
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = [{"id": 2}]
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_word_with_accent_using_search_term_with_accent(self):
        gql = self._get_graphql_search_user_query("camión")
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = [{"id": 2}]
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_word_without_accent_using_search_term_with_accent(self):
        gql = self._get_graphql_search_user_query("uniqué address vál")
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = [{"id": 2}]
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_word_with_accent_using_search_term_without_accent(self):
        gql = self._get_graphql_search_user_query("camion")
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = [{"id": 2}]
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_term_in_discord_fields(self):
        expected_result = [{"id": 2}]
        expected_result_and_search_term_array = [
            (expected_result, "only in discord user_name"),
            (expected_result, 1111),
            (expected_result, "only in discord alias"),
            (expected_result, "only in discord global_name"),
            (expected_result, "only in discord legacy_user_name"),
        ]
        for expected_result, search_term in expected_result_and_search_term_array:
            with self.subTest(search_term=search_term):
                gql = self._get_graphql_search_user_query(search_term)
                schema_result = schema.execute(gql)
                result = schema_result.data["searchUser"]
                self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_term_in_discord_fields_partial_integer_searched(self):
        expected_result = [{"id": 2}]
        search_term = "111"
        gql = self._get_graphql_search_user_query(search_term)
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        self.assertEqual(expected_result, result)

    def _get_graphql_search_user_query(self, search_term: str) -> str:
        return '{ searchUser(searchTerm: "' + str(search_term) + '") { id } }'
