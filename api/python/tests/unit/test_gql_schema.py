import unittest


from src.gql.schema import schema


class TestSchemaQuery(unittest.TestCase):
    maxDiff = None

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
            categories {
              id
              idUser
              idCategory
              category {
                id
                category
              }
            }
            discord {
              id
              idUser
              userName
              discriminator
              alias
              globalName
              legacyUserName
            }
            facebook {
              id
              idUser
              url
            }
            github {
              id
              idUser
              url
            }
            instagram {
              id
              idUser
              handle
            }
            linkedin {
              id
              idUser
              url
            }
            nicknames {
              id
              idUser
              nickname
            }
            notes {
              id
              idUser
              note
            }
            phones {
              id
              idUser
              phone
              description
            }
            telegram {
              id
              idUser
              userName
            }
            tiktok {
              id
              idUser
              userName
            }
            twitter {
              id
              idUser
              handle
            }
            urls {
              id
              idUser
              url
            }
            wallapop {
              id
              idUser
              url
              note
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
            "categories": [
                {
                    "id": 1,
                    "idUser": 2,
                    "idCategory": 1,
                    "category": {
                        "id": 1,
                        "category": "Family",
                    },
                },
                {
                    "id": 2,
                    "idUser": 2,
                    "idCategory": 2,
                    "category": {
                        "id": 2,
                        "category": "Work",
                    },
                },
            ],
            "discord": [
                {
                    "id": 1,
                    "idUser": 2,
                    "userName": "value only in discord user_name",
                    "discriminator": 111,
                    "alias": "value only in discord alias",
                    "globalName": "value only in discord global_name",
                    "legacyUserName": "value only in discord legacy_user_name",
                }
            ],
            "facebook": [
                {
                    "id": 1,
                    "idUser": 2,
                    "url": "https://www.facebook.com/unique_facebook_user.2/",
                }
            ],
            "github": [
                {
                    "id": 1,
                    "idUser": 2,
                    "url": "https://github.com/unique_github_user_2/",
                }
            ],
            "instagram": [
                {
                    "id": 1,
                    "idUser": 2,
                    "handle": "unique_instagram_user_2",
                }
            ],
            "linkedin": [
                {
                    "id": 1,
                    "idUser": 2,
                    "url": "https://www.linkedin.com/in/unique_linkedin_user_2",
                }
            ],
            "nicknames": [
                {
                    "id": 1,
                    "idUser": 2,
                    "nickname": "unique_nickname_user_2",
                }
            ],
            "notes": [
                {
                    "id": 1,
                    "idUser": 2,
                    "note": "Unique note user 2",
                }
            ],
            "phones": [
                {
                    "id": 1,
                    "idUser": 2,
                    "phone": 666666666,
                    "description": "Unique phone description user 2",
                }
            ],
            "telegram": [
                {
                    "id": 1,
                    "idUser": 2,
                    "userName": "unique_telegram_user_name_user_2",
                }
            ],
            "tiktok": [
                {
                    "id": 1,
                    "idUser": 2,
                    "userName": "unique_tiktok_user_name_user_2",
                }
            ],
            "twitter": [
                {
                    "id": 1,
                    "idUser": 2,
                    "handle": "unique_twitter_user_2",
                }
            ],
            "urls": [
                {
                    "id": 1,
                    "idUser": 2,
                    "url": "https://unique_url_user_2.com",
                }
            ],
            "wallapop": [
                {
                    "id": 1,
                    "idUser": 2,
                    "url": "https://unique_wallapop_url_user_2.com",
                    "note": "Unique wallapop note user 2",
                }
            ],
        }
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_sorts_results_by_ascending_name(self):
        gql = """
        {
          searchUser(searchTerm: "") {
            id
          }
        }
        """
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        self.assertEqual([{"id": 1}, {"id": 2}], result)

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
            (expected_result, "111"),
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

    def test_resolve_search_user_if_search_term_in_discord_is_partial_discriminator(self):
        expected_result = [{"id": 2}]
        search_term = "11"
        gql = self._get_graphql_search_user_query(search_term)
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_term_in_discord_is_excact_discriminator(self):
        expected_result = [{"id": 2}]
        search_term = "111"
        gql = self._get_graphql_search_user_query(search_term)
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_term_in_facebook(self):
        gql = self._get_graphql_search_user_query("unique_facebook")
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = [{"id": 2}]
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_term_in_github(self):
        gql = self._get_graphql_search_user_query("unique_github")
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = [{"id": 2}]
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_term_in_instagram(self):
        gql = self._get_graphql_search_user_query("unique_instagram")
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = [{"id": 2}]
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_term_in_linkedin(self):
        gql = self._get_graphql_search_user_query("unique_linkedin")
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = [{"id": 2}]
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_term_in_nickname(self):
        gql = self._get_graphql_search_user_query("unique_nickname")
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = [{"id": 2}]
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_term_in_note(self):
        gql = self._get_graphql_search_user_query("unique note")
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = [{"id": 2}]
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_term_in_phone_description(self):
        gql = self._get_graphql_search_user_query("unique phone description")
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = [{"id": 2}]
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_term_in_phone_is_exact_phone(self):
        gql = self._get_graphql_search_user_query("666666666")
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = [{"id": 2}]
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_term_in_phone_is_partial_phone(self):
        gql = self._get_graphql_search_user_query("66666")
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = [{"id": 2}]
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_term_in_users_categories(self):
        gql = self._get_graphql_search_user_query("work")
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = [{"id": 2}]
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_term_in_telegram(self):
        gql = self._get_graphql_search_user_query("unique_telegram")
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = [{"id": 2}]
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_term_in_tiktok(self):
        gql = self._get_graphql_search_user_query("unique_tiktok")
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = [{"id": 2}]
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_term_in_twitter(self):
        gql = self._get_graphql_search_user_query("unique_twitter")
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = [{"id": 2}]
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_term_in_url(self):
        gql = self._get_graphql_search_user_query("unique_url")
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = [{"id": 2}]
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_term_in_wallapop_url(self):
        gql = self._get_graphql_search_user_query("unique_wallapop_url")
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = [{"id": 2}]
        self.assertEqual(expected_result, result)

    def test_resolve_search_user_if_search_term_in_wallapop_note(self):
        gql = self._get_graphql_search_user_query("unique wallapop note")
        schema_result = schema.execute(gql)
        result = schema_result.data["searchUser"]
        expected_result = [{"id": 2}]
        self.assertEqual(expected_result, result)

    def _get_graphql_search_user_query(self, search_term: str) -> str:
        return '{ searchUser(searchTerm: "' + search_term + '") { id } }'
