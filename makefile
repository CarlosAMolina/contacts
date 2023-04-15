ROOT_PATHNAME=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
APP_PATHNAME=$(ROOT_PATHNAME)/contacts_reader

run:
	cd contacts_reader && cargo run carlos

build_for_debian:
	cd contacts_reader && docker run --rm -v $(APP_PATHNAME):/usr/src/myapp -w /usr/src/myapp rust cargo build --release

