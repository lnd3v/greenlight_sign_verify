RS_FILES := $(shell find src -type f -name '*.rs')
TARGET := ./target/debug/greenlight_sign_verify
all: run

$(TARGET): $(RS_FILES)
	cargo fmt
	cargo build

build: $(TARGET)

run: build
	$(TARGET)
