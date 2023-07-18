CONTAINER_DATA_PATHNAME=/home/postgres/data
DB_DUMP_FILE_NAME=contacts.sql
CONTAINER_DB_DUMP_FILE_PATHNAME=$(CONTAINER_DATA_PATHNAME)/$(DB_DUMP_FILE_NAME)
LOCAL_DB_DUMP_FILE_PATHNAME=/tmp/$(DB_DUMP_FILE_NAME)
CONTAINER_NAME=postgres-container
CONTAINER_VOLUME=db_postgres
DATABASE_NAME=contacts
IMAGE_NAME=postgres
IMAGE_VERSION=15.2
POSTGRES_USER=postgres
POSTGRES_PW=pw

pull-image:
	docker pull $(IMAGE_NAME):$(IMAGE_VERSION)

create-volume:
	docker volume create $(CONTAINER_VOLUME)

run:
	# PGDATA: tells PostgreSQL where it should store the database.
	docker run \
		--rm \
		-d \
		--name $(CONTAINER_NAME) \
		-p5432:5432 \
		-v $(CONTAINER_VOLUME):$(CONTAINER_DATA_PATHNAME) \
		-e POSTGRES_PASSWORD=$(POSTGRES_PW) \
		-e PGDATA=$(CONTAINER_DATA_PATHNAME) \
		$(IMAGE_NAME)

connect:
	docker exec -it $(CONTAINER_NAME) /bin/bash

connect-psql:
	docker exec -it $(CONTAINER_NAME) psql -U $(POSTGRES_USER)

export-db:
	# Init dump database
	docker exec $(CONTAINER_NAME) \
		pg_dump -U $(POSTGRES_USER) -d $(DATABASE_NAME) -f $(CONTAINER_DB_DUMP_FILE_PATHNAME)
	# Init export dumped database
	docker cp $(CONTAINER_NAME):$(CONTAINER_DB_DUMP_FILE_PATHNAME) $(LOCAL_DB_DUMP_FILE_PATHNAME)

import-db:
	# Init import database
	# Init drop database
	docker exec -it $(CONTAINER_NAME) \
		psql -U $(POSTGRES_USER) -c "DROP DATABASE IF EXISTS $(DATABASE_NAME);"
	# Init create database
	docker exec -it $(CONTAINER_NAME) \
		psql -U $(POSTGRES_USER) -c "CREATE DATABASE $(DATABASE_NAME);"
	# Copy dumped database file to the container
	docker cp  $(LOCAL_DB_DUMP_FILE_PATHNAME) $(CONTAINER_NAME):$(CONTAINER_DB_DUMP_FILE_PATHNAME)
	# Init import database
	docker exec -it $(CONTAINER_NAME) \
		psql -U $(POSTGRES_USER) --set ON_ERROR_STOP=on -f $(CONTAINER_DB_DUMP_FILE_PATHNAME) $(DATABASE_NAME)

stop:
	docker stop $(CONTAINER_NAME)

