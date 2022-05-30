
build := pkg
wasm := $(build)/image_bg.wasm

ALL: $(wasm)

src := $(shell find src -name '*.rs')

$(wasm): $(src) Cargo.toml
	wasm-pack build --target web
	jq '. + {type:"module"}' pkg/package.json > pkg/tmp.json
	mv pkg/tmp.json pkg/package.json

clean:
	rm -rf $(build)