ROOT_PATHNAME=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
SERVER_FOLDER_NAME=contacts
SERVER_PATHNAME=$(ROOT_PATHNAME)/$(SERVER_FOLDER_NAME)
CLI_PATHNAME=$(ROOT_PATHNAME)/cli
API_PORT=3030

build-server-for-debian:
	cd $(SERVER_PATHNAME) && docker run --rm -v $(SERVER_PATHNAME):/usr/src/myapp -w /usr/src/myapp rust cargo build --release

build-cli-for-debian:
	cd $(CLI_PATHNAME) && docker run --rm -v $(CLI_PATHNAME):/usr/src/myapp -w /usr/src/myapp rust cargo build --release

doc:
	cd $(SERVER_PATHNAME) && cargo doc && cargo doc --open

run-server:
	cd $(SERVER_PATHNAME) && cargo run &

stop-server:
	pkill contacts

call-return-error:
	curl \
		-X OPTIONS localhost:$(API_PORT)/questions \
		-H "Access-Control-Request-Method: PUT" \
		-H "Access-Control-Request-Headers: invalid-header" \
		-H "Origin: https://not-origin.io" \
		-verbose

get-contact-by-id:
	#curl "localhost:$(API_PORT)/contacts/86"
	cd cli && cargo run -- --id 86

get-contacts-all:
	curl "localhost:$(API_PORT)/contacts"

get-contacts-query:
	#curl "localhost:$(API_PORT)/contacts?query=arlos%20a"
	cd cli && cargo run -- arlos a

get-contacts-paginated:
	curl "localhost:$(API_PORT)/contacts?start=0&end=1"

run-db:
	make -f $(ROOT_PATHNAME)/makefile-db run

stop-db:
	make -f $(ROOT_PATHNAME)/makefile-db stop

run: run-db \
	start-server

