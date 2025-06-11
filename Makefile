TARGET = target/release/alex3

.PHONY: build
build $(TARGET):
	cargo build --release

.PHONY: test
test:
	cargo test

.PHONY: run
run: $(TARGET)
	./$(TARGET)