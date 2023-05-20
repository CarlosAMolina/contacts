doc:
	cd server && cargo doc && cargo doc --open

call-return-error:
	curl \
		-X OPTIONS localhost:3030/questions \
		-H "Access-Control-Request-Method: PUT" \
		-H "Access-Control-Request-Headers: invalid-header" \
		-H "Origin: https://not-origin.io" \
		-verbose

start-server:
	./target/debug/contacts &

stop-server:
	pkill contacts

get-contact:
	curl "localhost:3030/contacts/0"

get-contacts:
	curl "localhost:3030/contacts"

get-contacts-paginated:
	curl "localhost:3030/contacts?start=0&end=1"

