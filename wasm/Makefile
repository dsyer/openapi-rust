
srcs := src

build := pkg
target := target
wasm := $(build)/openapi_rust_bg.wasm

ALL: $(wasm)

src := $(wildcard $(srcs)/*.rs)

$(wasm): $(src) $(crate)
	wasm-pack build --target web

clean:
	rm -rf $(build) $(target)