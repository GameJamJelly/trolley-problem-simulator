all: build

build:
	cargo build

run:
	cargo run

clean:
	cargo clean

build-web:
	trunk build

run-web:
	python -m http.server -d dist 3000

clean-web:
	trunk clean
