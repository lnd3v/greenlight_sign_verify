all: run

./target/debug/tmp: ./src/main.rs
	cargo fmt
	cargo build

build: ./target/debug/tmp
	echo 'build'

run: build
	./target/debug/tmp