API_CONTAINER_IP=172.20.0.7
API_PORT=5000
CLI_IMAGE_NAME=contacts-cli-python
CLI_CONTAINER_NAME=$(CLI_IMAGE_NAME)-container
API_GRAPHQL_URL=http://$(API_CONTAINER_IP):$(API_PORT)/graphql
NETWORK_NAME=contacts-network

docker-build:
	docker build -t $(CLI_IMAGE_NAME):1.0 .

docker-run:
	docker run \
		--rm \
		-dit \
		--net=$(NETWORK_NAME) \
		--name $(CLI_CONTAINER_NAME) \
		--env GRAPHQL_URL=$(API_GRAPHQL_URL) \
		$(CLI_IMAGE_NAME):1.0

docker-connect:
	docker exec -it $(CLI_CONTAINER_NAME) python main.py

# [pass argument](https://stackoverflow.com/a/2826069)
docker-search-term:
	docker exec -it $(CLI_CONTAINER_NAME) python main.py $(term)

# [pass argument](https://stackoverflow.com/a/2826069)
docker-search-id:
	docker exec -it $(CLI_CONTAINER_NAME) python main.py -i $(term)

test:
	python -m unittest discover -s tests

run-export:
	python src/main.py -e

