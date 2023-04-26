TEAM=xkloub03_xnguye27_xmorav48
ZIP_FILE=$(TEAM).zip

all: build profile

pack:

clean:
	rm -rf target ../doc/ $(ZIP_FILE)

build:
	cargo build --release --package calc

run:
	cargo run --release --package calc

pack:
	mkdir $(TEAM) \
	&& cd $(TEAM) \
	&& git clone https://github.com/FIT-IVS-segmentation-fault-core-dumped/IVS-projekt-2 repo \
	&& echo "Getting installers" \
	&& mkdir install\
	&& cd install \
	&& wget https://github.com/FIT-IVS-segmentation-fault-core-dumped/IVS-projekt-2/releases/download/v0.1.0/fitcalc-0.1.0-x86_64_win10.msi \
	&& wget https://github.com/FIT-IVS-segmentation-fault-core-dumped/IVS-projekt-2/releases/download/v0.1.0/fitcalc_0.1.0_amd64_ubuntu2004.deb \
	&& wget https://github.com/FIT-IVS-segmentation-fault-core-dumped/IVS-projekt-2/releases/download/v0.1.0/fitcalc-git-0.1.1-0-x86_64.pkg.tar.zst \
	&& wget https://github.com/FIT-IVS-segmentation-fault-core-dumped/IVS-projekt-2/releases/download/v0.1.0/profiling_0.1.0_amd64_ubuntu2004.deb \
	&& cd .. \
	&& echo "Getting documentation" \
	&& mkdir doc \
	&& cd .. \
	&& cargo doc --package math --no-deps --target-dir $(TEAM)/doc \
	&& mv $(TEAM)/doc/doc/* $(TEAM)/doc/ \
	&& echo "Zipping" \
	&& zip -r $(ZIP_FILE) $(TEAM) \
	&& echo "Clean up" \
	&& rm -rf $(TEAM)


doc:
	cargo doc --package math --no-deps --target-dir ../doc
	mv ../doc/doc/* ../doc/

profile:
	cargo build --release --package profiling && echo "Profiling program is built in /target/release/profiling"

install:
	install -D target/release/calc $(DESTDIR)/usr/bin/fitcalc
	install -D assets/fitcalc.desktop $(DESTDIR)/usr/share/applications/fitcalc.desktop
	install -D assets/fitcalc_128x128.png $(DESTDIR)/usr/share/icons/fitcalc_128x128.png

