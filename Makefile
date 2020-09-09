.PHONY: build
build: target/release/hatter

.PHONY: docs
docs: target/release/hatter docs/index.hat
	cargo run --release docs/index.hat > docs/index.html

.PHONY: serve
serve: docs
	serve -r docs/

.PHONY: watch
watch: docs
	@fswatch --version > /dev/null
	@fish --version > /dev/null
	@make serve &
	@fish -c 'while true; fswatch -1 docs/*.hat || break && make docs; end'

.PHONY: clean
clean:
	rm -rf target

target/release/hatter: src/*.rs src/**/*.rs
	cargo build --release
