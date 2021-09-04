.PHONY: aoclib aoclib-dbg clean_all build_all c cpp rust

PREFIX?=/usr/local

rust/target/release/libaocdata.so rust/aocdata/adventofcode.h:
	cargo build --manifest-path rust/Cargo.toml -p aocdata --release

rust/target/debug/libaocdata.so:
	cargo build --manifest-path rust/Cargo.toml -p aocdata

aoclib: rust/target/release/libaocdata.so rust/aocdata/adventofcode.h

aoclib-dbg: rust/target/debug/libaocdata.so

install: aoclib
	install rust/target/release/libaocdata.so $(PREFIX)/lib
	install rust/aocdata/adventofcode.h $(PREFIX)/include

install-dbg: aoclib-dbg
	install rust/target/debug/libaocdata.so $(PREFIX)/lib
	install rust/aocdata/adventofcode.h $(PREFIX)/include

uninstall:
	rm -f $(PREFIX)/lib/libaocdata.so
	rm -f $(PREFIX)/include/adventofcode.h

clean_all:
	@cargo clean --manifest-path rust/Cargo.toml
	@rm -f rust/aocdata/adventofcode.h
	@make -C c/ clean
	@make -C cpp/ clean

c:
	make -C c gcc clang
	make -C c/build -j
	make -C c/clang-build -j

cpp:
	make -C cpp gcc clang
	make -C cpp/build -j
	make -C cpp/clang-build -j

rust:
	cargo build --manifest-path rust/Cargo.toml -p advent-of-code-rs --release

build_all: c cpp rust
