# TODO

## API python

[ ] Models varchar must specify number of characters, because alembic is creating the columns as `varchar` instead ov `varchar(x)`
[ ] `id` column of all tables must be autoincrement?

## Rust. To implement

[x] Logs
[x] - Save API docker logs in volume
[ ] - Logs rotation
[ ] Docker compose
[ ] Models
[ ] - Discord
[ ] - Linkedin
[x] Login
[ ] - Login errors:
[ ]   - ArgonLibraryError
[ ]   - CannotDecryptToken
[x] CRUD tables
[ ] - Modify tables
[ ] - Error if row not found
[x] - Search: find accentuated characters if query no accentuated and vice versa.
[x] Drop
[ ] - contacts.csv
[x] Handle-errors
[ ] - lib.rs. Implement error events and improve code like `return_error`
[x] Front
[x] - Custom error messages.
[x] Database
[x] - Change user table to use serial column

