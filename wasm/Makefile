
srcs := src

build := pkg
target := target
wasm := $(build)/deployment_bg.wasm

ALL: $(wasm)

src := $(wildcard $(srcs)/*.rs)

$(wasm): $(src)
	wasm-pack build --target web
	jq '. + {type:"module"}' pkg/package.json > pkg/tmp.json
	mv pkg/tmp.json pkg/package.json

clean:
	rm -rf $(build) $(target)