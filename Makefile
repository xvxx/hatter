.PHONY: build
build: target/release/hatter

.PHONY: docs
docs: target/release/hatter docs/index.hat
	cargo run --release docs/index.hat > docs/index.html

.PHONY: serve
serve: docs
	serve -r docs/

.PHONY: clean
clean:
	rm -rf target

target/release/hatter: src/*.rs src/**/*.rs
	cargo build --release
