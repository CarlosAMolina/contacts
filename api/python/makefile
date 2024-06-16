alembic-upgrade-sqlite:
	export DB_URL=sqlite:////tmp/contacts.sqlite3 && alembic upgrade heads

docker-build:
	docker build -t contacts:1.0 .

# FLASK_HOST: to detect IP used by the Docker container, when the container is running,
# execute `docker network inspect bridge | grep IPv4Address`
# https://stackoverflow.com/a/58138250
docker-run:
	docker run \
		--env DB_URL=sqlite:///contacts.sqlite3 \
		--env FLASK_HOST=172.17.0.2 \
		-v /tmp/contacts.sqlite3:/contacts/contacts.sqlite3 \
		-p 5000:5000 \
		-d \
		--rm \
		-it contacts:1.0

run-postgres:
	export DB_URL=postgresql+psycopg://postgres:pw@localhost:5432/contacts && python main.py

run-sqlite:
	export DB_URL=sqlite:////tmp/contacts.sqlite3 && python main.py

run-sqlite-test-db:
	export DB_URL=sqlite:////tmp/contacts.test.sqlite3 && python main.py

test:
	python -m unittest discover -s tests

test-filter:
	python -m unittest discover -s tests -k test_resolve_search_user_if_search_term_in_users_categories

