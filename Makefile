.PHONY: build
build: target/release/hatter

.PHONY: clean
clean:
	rm -rf target

target/release/hatter: src/*.rs src/**/*.rs
	cargo build --release
