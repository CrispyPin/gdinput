CRATE=gdinput
ADDON=gdinput
LIBNAME=lib$(CRATE).so

default: debug
debug:
	cd $(CRATE) && cargo build
	ln -sf ../../../../../$(CRATE)/target/debug/$(LIBNAME) project/addons/$(ADDON)/bin/linux/$(LIBNAME)

r: release
release:
	cd $(CRATE) && cargo build --release
	ln -sf ../../../../../$(CRATE)/target/release/$(LIBNAME) project/addons/$(ADDON)/bin/linux/$(LIBNAME)

c: clippy
clippy:
	cd $(CRATE) && cargo clippy
