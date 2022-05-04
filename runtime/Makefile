
srcs := src

build := target
exe := $(build)/debug/openapi-rust-runtime

ALL: $(exe)

src := $(wildcard $(srcs)/*.rs)

$(exe): $(src)
	cargo build

clean:
	rm -rf $(build) $(openapi)