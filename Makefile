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
	RUST_LOG=debug OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317 DYNAMODB_USE_LOCAL="true" DYNAMODB_ENDPOINT="http://0.0.0.0:3000" AWS_ACCESS_KEY_ID="identity" AWS_SECRET_ACCESS_KEY="credential" AWS_REGION="eu-west-1" ./target/release/$(prog)

dev:
	RUST_LOG=debug OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317 DYNAMODB_USE_LOCAL="true" DYNAMODB_ENDPOINT="http://0.0.0.0:3000" AWS_ACCESS_KEY_ID="identity" AWS_SECRET_ACCESS_KEY="credential" AWS_REGION="eu-west-1" ./target/debug/$(prog)

install:
	cp target/$(target)/$(prog) ~/bin/$(prog)-$(extension)

dynamodb:
	java -Djava.library.path=./dynamodb/DynamoDBLocal_lib -jar ./dynamodb/DynamoDBLocal.jar -sharedDb

install-dynamodb:
	sh ./install_local_dynamodb.sh

# testing purposes - http://localhost:4317/v1/traces
otel:
	docker run \
		-v "${PWD}/docker-otel.local.yml":/docker-otel.local.yml \
		-p 4317:4317 \
		otel/opentelemetry-collector \
		--config docker-otel.local.yml;

all: build install
 
help:
	@echo "usage: make $(prog) [debug=1]"