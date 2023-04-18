all: build profile

pack:

clean:
	rm -rf target ../doc/

build:
	cargo build --release --package calc

run:
	cargo run --release --package calc


doc:
	cargo doc --package math --no-deps --target-dir ../doc
	mv ../doc/doc/* ../doc/

profile:
	cargo build --release --package profiling
