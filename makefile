RS_FILES := $(shell find src -type f -name '*.rs')

all: run

./target/debug/tmp: $(RS_FILES)
	cargo fmt
	cargo build

build: ./target/debug/tmp
	echo 'build'

run: build
	./target/debug/tmp
