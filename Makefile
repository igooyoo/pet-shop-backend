build:
	@cargo build

clean:
	@cargo clean

TESTS = ""
test:
	@cargo test $(TESTS) --offline --lib -- --color=always --nocapture

docs: build
	@cargo doc --no-deps

style-check:
	@rustup component add rustfmt 2> /dev/null
	cargo fmt --all -- --check

lint:
	@rustup component add clippy 2> /dev/null
	cargo clippy --all-targets --all-features -- -D warnings

dev:
	cargo run

alpine:
	@sudo docker build -t ghcr.io/aircampi/auth-api-alpine -f ./builder/alpine/Dockerfile .

debian:
	@sudo docker build -t ghcr.io/aircampi/auth-api-debian -f ./builder/debian/Dockerfile .

container:
	docker-compose -f ./builder/alpine/docker-compose.yml up & 

.PHONY: build test docs style-check lint alpine debian
