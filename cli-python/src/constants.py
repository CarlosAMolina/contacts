GRAPHQL_URL = "http://127.0.0.1:5000/graphql"

BODY_TERM_SEARCH = """
{
  usersWithTerm(searchTerm: "{SEARCH_TERM}") {
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
