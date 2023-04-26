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

install:
	install -D target/release/calc $(DESTDIR)/usr/bin/calc
	install -D assets/calc.desktop $(DESTDIR)/usr/share/applications/calc.desktop
	install -D assets/calc_128x128.png $(DESTDIR)/usr/share/icons/calc_128x128.png

