GRAPHQL_URL = "http://127.0.0.1:5000/graphql"

BODY_ID_SEARCH = """
{
  user(userId: {SEARCH_TERM}) {
    name
    surname
    addresses {
      address
    }
    categories {
      category {
        category
      }
    }
    discord {
      userName
      discriminator
      alias
      globalName
      legacyUserName
    }
    emails {
      email
    }
    facebook {
      url
    }
    github {
      url
    }
    instagram {
      handle
    }
    linkedin {
      url
    }
    nicknames {
      nickname
    }
    notes {
      note
    }
    phones {
      phone
      description
    }
    telegram {
      userName
    }
    tiktok {
      userName
    }
    twitter {
      handle
    }
    urls {
      url
    }
    wallapop {
      url
      note
    }
  }
}
"""

BODY_ALL_SEARCH = BODY_ID_SEARCH.replace("user(userId: {SEARCH_TERM})", "usersWithTerm(searchTerm: '')")


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
