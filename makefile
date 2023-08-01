ROOT_PATH_NAME=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
API_FOLDER_NAME=contacts
API_PATH_NAME=$(ROOT_PATH_NAME)/$(API_FOLDER_NAME)
CLI_FOLDER_NAME=cli
CLI_PATH_NAME=$(ROOT_PATH_NAME)/$(CLI_FOLDER_NAME)
API_PORT=3030
API_IMAGE_NAME=contacts-api
API_CONTAINER_NAME=$(API_IMAGE_NAME)-container
CLI_IMAGE_NAME=contacts-cli
CLI_CONTAINER_NAME=$(CLI_IMAGE_NAME)-container
NETWORK_NAME=contacts-network

build-api-docker:
	cd $(API_PATH_NAME) && docker build -t $(API_IMAGE_NAME) .

build-cli-docker:
	cd $(CLI_PATH_NAME) && docker build -t $(CLI_IMAGE_NAME) .

build-server-for-debian:
	cd $(API_PATH_NAME) && docker run --rm -v $(API_PATH_NAME):/usr/src/myapp -w /usr/src/myapp rust cargo build --release

build-cli-for-debian:
	cd $(CLI_PATH_NAME) && docker run --rm -v $(CLI_PATH_NAME):/usr/src/myapp -w /usr/src/myapp rust cargo build --release

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

run-cli-docker:
	docker run \
		-it \
		--rm \
		--name $(CLI_CONTAINER_NAME) \
		--net=$(NETWORK_NAME) \
		$(CLI_IMAGE_NAME) \
		carlos a

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
	make -f $(ROOT_PATH_NAME)/makefile-db run

stop-db:
	make -f $(ROOT_PATH_NAME)/makefile-db stop

run: run-db \
	run-api-docker
