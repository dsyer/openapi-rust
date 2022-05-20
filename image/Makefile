
build := pkg
target := target
schema := $(target)/k8s.json
crate := $(target)/rust
wasm := $(build)/image_bg.wasm

ALL: $(schema) $(wasm)

src := $(shell find src -name '*.rs')

$(wasm): $(src) $(crate)/Cargo.toml
	wasm-pack build --target web

$(crate)/Cargo.toml: $(schema)
	./mvnw generate-sources
	cp -r target/rust/src/models src

$(schema):
	mkdir -p $(target)
	kubectl get --raw /openapi/v2 | \
		jq 'with_entries(select([.key] | inside(["definitions", "components", "info", "swagger", "openapi"]))) + {paths:{}}' \
		> $(schema)
	jq 'with_entries(select([.key] | inside(["info", "swagger", "openapi"])))' $(schema) > target/info.json
	jq '.definitions | with_entries( select(.key|startswith("com.example"))) | with_entries(.key|= sub("com.example.";"")) | with_entries(.value += {"x-implements":[if(.key|test(".*List.*")) then "io.kubernetes.client.common.KubernetesListObject" else "io.kubernetes.client.common.KubernetesObject" end]})' $(schema) > target/definitions.json
	jq -s '.[0] + {definitions:.[1]} + {paths:{}}' target/info.json target/definitions.json  > $(schema)
	sed -i -e 's,#/definitions/io.k8s.apimachinery.pkg.apis.meta.,#/definitions/,' -e 's,#/definitions/com.example.,#/definitions/,' $(schema)

clean:
	rm -rf $(build) $(openapi) $(target)