.PHONY: gcc gcc-debug clang clang-debug

BUILD?="Unix Makefiles"

build:
	@mkdir -p $@

clang-build:
	@mkdir -p $@

debug:
	@mkdir -p $@

clang-debug:
	@mkdir -p $@

gcc: build
	@cmake -B $^ -S . -DCMAKE_BUILD_TYPE=Release -G $(BUILD)

gcc-dbg: debug
	@cmake -S . -B $^ -DCMAKE_BUILD_TYPE=Debug -G $(BUILD)

clang: clang-build
	@CC=clang CXX=clang++ LD=lld cmake -B $^ -S . -DCMAKE_BUILD_TYPE=Release -G $(BUILD)

clang-dbg: clang-debug
	@CC=clang CXX=clang++ LD=lld cmake -S . -B $^ -DCMAKE_BUILD_TYPE=Debug -G $(BUILD)

clean:
	@rm -rf build debug clang-build clang-debug
