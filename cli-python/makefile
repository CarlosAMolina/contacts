docker-build:
	docker build -t contacts-cli:1.0 .

docker-run:
	docker run \
		--rm \
		-dit \
		--name contacts-cli \
		contacts-cli:1.0

docker-connect:
	docker exec -it contacts-cli python main.py
