API_CONTAINER_IP=172.20.0.7
API_IMAGE_NAME=contacts-api-python
API_CONTAINER_NAME=$(API_IMAGE_NAME)-container
API_PORT=5000
NETWORK_NAME=contacts-network

alembic-upgrade-sqlite:
	export DB_URL=sqlite:////tmp/contacts.sqlite3 && alembic upgrade heads

docker-build:
	docker build -t $(API_IMAGE_NAME):1.0 .

# FLASK_HOST: to detect IP used by the Docker container, when the container is running,
# execute `docker network inspect bridge | grep IPv4Address`
# https://stackoverflow.com/a/58138250
docker-run:
	docker run \
		--rm \
		-dit \
		--name $(API_CONTAINER_NAME) \
		-p $(API_PORT):$(API_PORT)\
		--net=$(NETWORK_NAME) \
		--ip=$(API_CONTAINER_IP) \
		-v /tmp/contacts.sqlite3:/contacts/contacts.sqlite3 \
		--env DB_URL=sqlite:///contacts.sqlite3 \
		--env FLASK_HOST=$(API_CONTAINER_IP) \
		$(API_IMAGE_NAME):1.0

run-postgres:
	export DB_URL=postgresql+psycopg://postgres:pw@localhost:5432/contacts && python main.py

run-sqlite:
	export DB_URL=sqlite:////tmp/contacts.sqlite3 && python main.py

run-sqlite-test-db:
	export DB_URL=sqlite:////tmp/contacts.test.sqlite3 && python main.py

test:
	python -m unittest discover -s tests

test-filter:
	python -m unittest discover -s tests -k test_resolve_users_with_term_if_search_term_in_users_categories

