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
	install -D target/release/calc $(DESTDIR)/usr/bin/fitcalc
	install -D assets/fitcalc.desktop $(DESTDIR)/usr/share/applications/fitcalc.desktop
	install -D assets/fitcalc_128x128.png $(DESTDIR)/usr/share/icons/fitcalc_128x128.png

