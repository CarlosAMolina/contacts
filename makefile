ROOT_PATHNAME=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
API_FOLDER_NAME=contacts
API_PATH_NAME=$(ROOT_PATHNAME)/$(API_FOLDER_NAME)
CLI_PATHNAME=$(ROOT_PATHNAME)/cli
API_PORT=3030
API_IMAGE_NAME=contacts-api
API_CONTAINER_NAME=$(API_IMAGE_NAME)-container
NETWORK_NAME=contacts-network

build-api-docker:
	cd $(API_PATH_NAME) && docker build -t $(API_IMAGE_NAME) .

build-server-for-debian:
	cd $(API_PATH_NAME) && docker run --rm -v $(API_PATH_NAME):/usr/src/myapp -w /usr/src/myapp rust cargo build --release

build-cli-for-debian:
	cd $(CLI_PATHNAME) && docker run --rm -v $(CLI_PATHNAME):/usr/src/myapp -w /usr/src/myapp rust cargo build --release

doc:
	cd $(API_PATH_NAME) && cargo doc && cargo doc --open

run-server:
	cd $(API_PATH_NAME) && cargo run &

run-api-docker:
	docker run \
		--rm \
		-d \
		--name $(API_CONTAINER_NAME) \
		-p$(API_PORT):$(API_PORT)\
		--net=$(NETWORK_NAME) \
		$(API_IMAGE_NAME)

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
	run-api-docker
