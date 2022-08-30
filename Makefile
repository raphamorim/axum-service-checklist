prog :=service
debug ?=
$(info DEBUG: $(debug))

ifdef debug
	release :=
	target :=debug
	extension :=debug
else
	release :=--release
	target :=release
	extension :=
endif

lint:
	cargo fmt -- --check --color always
	cargo clippy --all-targets -- -D warnings

build:
	cargo build $(release)

docker-build:
	docker build -t service -f ./Dockerfile .

# Need to install cargo watch by cargo install cargo-watch
watch:
	cargo watch -- make dev

start:
	./target/release/$(prog)

dev:
	make build debug=1
	RUST_LOG=debug ./target/debug/$(prog)

install:
	cp target/$(target)/$(prog) ~/bin/$(prog)-$(extension)

all: build install
 
help:
	@echo "usage: make $(prog) [debug=1]"