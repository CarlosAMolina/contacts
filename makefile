ROOT_PATH_NAME=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
API_FOLDER_NAME=api
API_ROOT_FOLDER_PATH_NAME=$(ROOT_PATH_NAME)/$(API_FOLDER_NAME)
CLI_BINARY_NAME=cli
CLI_FOLDER_NAME=cli
CLI_ROOT_FOLDER_PATH_NAME=$(ROOT_PATH_NAME)/$(CLI_FOLDER_NAME)
API_PORT=3030
API_IMAGE_NAME=contacts-api
API_CONTAINER_NAME=$(API_IMAGE_NAME)-container
API_CONTAINER_IP=172.20.0.6
API_CONTAINER_LOGS_PATH_NAME=/logs
API_CONTAINER_LOGS_VOLUME_NAME=$(API_IMAGE_NAME)-logs
CLI_IMAGE_NAME=contacts-cli
CLI_CONTAINER_NAME=$(CLI_IMAGE_NAME)-container
NETWORK_NAME=contacts-network


build-api-docker-image:
	cd $(API_ROOT_FOLDER_PATH_NAME) && docker build -t $(API_IMAGE_NAME) .

build-cli-debian-binary:
	docker build -t $(CLI_IMAGE_NAME) $(CLI_ROOT_FOLDER_PATH_NAME)
	docker run --rm -v $(CLI_ROOT_FOLDER_PATH_NAME):/opt/mount --entrypoint cp $(CLI_IMAGE_NAME) /app/cli /opt/mount/

doc:
	cd $(API_ROOT_FOLDER_PATH_NAME) && cargo doc && cargo doc --open

run-api-cargo:
	cd $(API_ROOT_FOLDER_PATH_NAME) && cargo run &

# TODO try to use only one internal port for the db to use by the api
run-api-docker:
	docker run \
		--rm \
		-d \
		--name $(API_CONTAINER_NAME) \
		-p$(API_PORT):$(API_PORT)\
		--net=$(NETWORK_NAME) \
		--ip=$(API_CONTAINER_IP) \
		--mount type=volume,source=$(API_CONTAINER_LOGS_VOLUME_NAME),target=$(API_CONTAINER_LOGS_PATH_NAME) \
		$(API_IMAGE_NAME)

stop-api-cargo:
	pkill api

call-return-error:
	curl \
		-X OPTIONS localhost:$(API_PORT)/questions \
		-H "Access-Control-Request-Method: PUT" \
		-H "Access-Control-Request-Headers: invalid-header" \
		-H "Origin: https://not-origin.io" \
		-verbose

add-account:
	curl --location --request POST 'localhost:3030/contacts' \
	--header 'Content-Type: application/json' \
	--data-raw '{ "user_name": "John", "user_surname": "Doe Do", "nicknames": ["Johnny", "Joy"], "phones": [ {"value": 666111222, "description": "Work"}, {"value": 666111333} ], "categories_id": [1, 2], "addresses": ["address 1", "address 2"], "emails": ["john@mail.com", "john2@mail.com"], "urls": ["john-home.com", "john-music.com"], "facebook_urls": ["facebook/John", "facebook/John2"], "twitter_handles": ["JohnT", "JohnT2"], "instagram_handles": ["JohnnyIns", "JohnnyIns2"], "note": "Jane´s brother" }'

# TODO implement
#get-contacts-all:
#	curl "localhost:$(API_PORT)/contacts"
#
# TODO implement
#get-contacts-paginated:
#	curl "localhost:$(API_PORT)/contacts?start=0&end=1"

get-invented-route:
	curl "localhost:$(API_PORT)/invented"

get-contacts-query:
	#curl "localhost:$(API_PORT)/contacts?query=arlos%20a"
	#cd $(CLI_ROOT_FOLDER_PATH_NAME) && cargo run -- arlos a
	cd $(CLI_ROOT_FOLDER_PATH_NAME) && ./$(CLI_BINARY_NAME) arlos a

get-contacts-query-long-format:
	#curl "localhost:$(API_PORT)/contacts?query=arlos%20a"
	#cd $(CLI_ROOT_FOLDER_PATH_NAME) && cargo run -- arlos a -f long
	cd $(CLI_ROOT_FOLDER_PATH_NAME) && ./$(CLI_BINARY_NAME) arlos a -f long

get-contact-by-id:
	#curl "localhost:$(API_PORT)/contacts/86"
	#cd $(CLI_ROOT_FOLDER_PATH_NAME) && cargo run -- --id 86
	cd $(CLI_ROOT_FOLDER_PATH_NAME) && ./$(CLI_BINARY_NAME) --id 86

run-db-docker:
	make -f $(ROOT_PATH_NAME)/makefile-db run-docker

stop-db-docker:
	make -f $(ROOT_PATH_NAME)/makefile-db stop-docker

stop-api-docker:
	docker stop $(API_CONTAINER_NAME)

test-api:
	cd $(API_ROOT_FOLDER_PATH_NAME) && cargo test

test-api-integration:
	cd $(API_ROOT_FOLDER_PATH_NAME)/integration-tests && cargo run

test-cli:
	cd $(CLI_ROOT_FOLDER_PATH_NAME) && cargo test

wait-2-seconds:
	sleep 2

run-cli-binary: get-contacts-query \
	get-contacts-query-long-format \
	get-contact-by-id

clean-unrequied-images:
	docker image prune -f
	#docker rmi $(shell docker images rust -aq)

build: build-api-docker-image \
	build-cli-debian-binary

run: run-db-docker \
	run-api-docker \
	wait-2-seconds \
	run-cli-binary

stop: stop-api-docker \
	stop-db-docker

deploy: stop \
	build \
	clean-unrequied-images \
	run

logs-api-docker:
	tail -f $(shell docker volume inspect $(API_CONTAINER_LOGS_VOLUME_NAME) --format '{{.Mountpoint}}')/*

test: test-api \
	test-cli \
	test-api-integration
