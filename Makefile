.PHONY: build
build: target/release/hatter

.PHONY: docs
docs: target/release/hatter docs/index.hat
	@cargo run -q --release docs/index.hat > docs/index.html

.PHONY: check
check:
	@(which -s pulldown-cmark) || (echo "Need pulldown-cmark installed\nRun: cargo install pulldown-cmark"; exit 1)
	@(which -s serve) || (echo "Need serve installed"; exit 1)
	@(which -s fswatch) || (echo "Need fswatch installed"; exit 1)
	@(which -s fish) || (echo "Need fish shell installed"; exit 1)

.PHONY: serve
serve: docs check
	@serve -r docs/

.PHONY: watch
watch: docs check
	@make serve &
	@open 'http://localhost:5000/'
	@fish -c 'while true; fswatch -1 docs/*.hat || break && make docs; end'

.PHONY: clean
clean:
	rm -rf target

target/release/hatter: src/*.rs src/**/*.rs
	cargo build --release
