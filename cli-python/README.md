## Bash configuration

Add to your ~/.bashrc file:

```bash
# Contacts
c () {
    cd ~/Software/contacts/cli-python/
    make docker-search-term term="$@"
    cd - > /dev/null
}
ci () {
    cd ~/Software/contacts/cli-python/
    make docker-search-id term="$@"
    cd - > /dev/null
}
```

## Run

```bash
make docker-build
make docker-run
make docker-connect
```

## Resources

[Docker and Python CLI](https://medium.com/swlh/dockerize-your-python-command-line-program-6a273f5c5544)
[Python GraphQL request](https://www.geeksforgeeks.org/get-and-post-requests-in-graphql-api-using-python-requests/)
