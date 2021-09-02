.PHONY: gcc gcc-debug clang clang-debug

build:
	@mkdir -p $@

clang-build:
	@mkdir -p $@

debug:
	@mkdir -p $@

clang-debug:
	@mkdir -p $@

gcc: build
	@cmake -B $^ -S . -DCMAKE_BUILD_TYPE=Release

gcc-dbg: debug
	@cmake -S . -B $^ -DCMAKE_BUILD_TYPE=Debug

clang: clang-build
	@CC=clang CXX=clang++ LD=lld cmake -B $^ -S . -DCMAKE_BUILD_TYPE=Release

clang-dbg: clang-debug
	@CC=clang CXX=clang++ LD=lld cmake -S . -B $^ -DCMAKE_BUILD_TYPE=Debug

clean:
	@rm -rf build debug clang-build clang-debug
