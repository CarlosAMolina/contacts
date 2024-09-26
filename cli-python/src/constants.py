GRAPHQL_URL = "http://127.0.0.1:5000/graphql"

BODY_ID_SEARCH = """
{
  user(userId: {SEARCH_TERM}) {
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
