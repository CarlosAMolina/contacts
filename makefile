ROOT_PATHNAME=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
APP_FOLDER_NAME=contacts
APP_PATHNAME=$(ROOT_PATHNAME)/$(APP_FOLDER_NAME)

build_for_debian:
	cd $(APP_PATHNAME) && docker run --rm -v $(APP_PATHNAME):/usr/src/myapp -w /usr/src/myapp rust cargo build --release

doc:
	cd $(APP_PATHNAME) && cargo doc && cargo doc --open

start-server:
	cd $(APP_PATHNAME) && cargo run &

stop-server:
	pkill contacts

call-return-error:
	curl \
		-X OPTIONS localhost:3030/questions \
		-H "Access-Control-Request-Method: PUT" \
		-H "Access-Control-Request-Headers: invalid-header" \
		-H "Origin: https://not-origin.io" \
		-verbose

get-contact-by-id:
	curl "localhost:3030/contacts/0"

get-contacts-all:
	curl "localhost:3030/contacts"

get-contacts-query:
	curl "localhost:3030/contacts?query=arlos"

get-contacts-paginated:
	curl "localhost:3030/contacts?start=0&end=1"

run-db:
	make -f $(ROOT_PATHNAME)/makefile-db run

