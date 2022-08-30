prog :=service
debug ?=
$(info DEBUG: $(debug))

.PHONY: dynamodb

ifdef debug
	release :=
	target :=debug
	extension :=debug
else
	release :=--release
	target :=release
	extension :=
endif

build:
	cargo build $(release)

docker-build:
	docker build -t service -f ./Dockerfile .

start:
	./target/release/$(prog)

dev:
	./target/debug/$(prog)

install:
	cp target/$(target)/$(prog) ~/bin/$(prog)-$(extension)

dynamodb:
	java -Djava.library.path=./dynamodb/DynamoDBLocal_lib -jar ./dynamodb/DynamoDBLocal.jar -sharedDb

install-dynamodb:
	sh ./install_local_dynamodb.sh

all: build install
 
help:
	@echo "usage: make $(prog) [debug=1]"