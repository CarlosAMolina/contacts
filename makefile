ROOT_PATHNAME=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
APP_FOLDER_NAME=contacts
APP_PATHNAME=$(ROOT_PATHNAME)/$(APP_FOLDER_NAME)

search_term:
	cd $(APP_PATHNAME) && cargo run carlos

export:
	cd $(APP_PATHNAME) && cargo run export 11

build_for_debian:
	cd $(APP_PATHNAME) && docker run --rm -v $(APP_PATHNAME):/usr/src/myapp -w /usr/src/myapp rust cargo build --release

