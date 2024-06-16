import requests

URL = "http://127.0.0.1:5000/graphql"

body = """
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

response = requests.post(url=URL, json={"query": body})
print("response status code: ", response.status_code)
if response.status_code == 200:
    print("response : ", response.json())
else:
    raise ValueError(f"GraphQL response: {response.content}")
