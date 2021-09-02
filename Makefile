.PHONY: aoclib clean_all build_all c cpp rust

PREFIX?=/usr/local

rust/target/release/libaocdata.so rust/aocdata/adventofcode.h:
	cargo build --manifest-path rust/Cargo.toml -p aocdata --release

aoclib: rust/target/release/libaocdata.so rust/aocdata/adventofcode.h

install: aoclib
	install rust/target/release/libaocdata.so $(PREFIX)/lib
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
