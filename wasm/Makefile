
srcs := src

build := pkg
target := target
openapi := openapi
schema := $(openapi)/k8s.json
crate := $(openapi)/Cargo.toml
wasm := $(build)/openapi_rust_bg.wasm

ALL: $(schema) $(wasm)

src := $(wildcard $(srcs)/*.rs)

$(wasm): $(src) $(crate)
	wasm-pack build --target web

$(crate): $(schema)
	jbang org.openapitools:openapi-generator-cli:6.0.0-beta generate -g rust -o $(openapi) -i $(schema)
	sed -i -e 's/version = "v/version ="/' $(openapi)/Cargo.toml

$(schema):
	mkdir -p $(openapi)
	kubectl get --raw /openapi/v2 | \
		jq 'with_entries(select([.key] | inside(["definitions", "components", "info", "swagger", "openapi"]))) + {paths:{}}' \
		> $(schema)

clean:
	rm -rf $(build) $(openapi) $(target)