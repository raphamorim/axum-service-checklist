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

all: build install
 
help:
	@echo "usage: make $(prog) [debug=1]"