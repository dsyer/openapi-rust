This project shows how to generate [Rust](https://www.rust-lang.org/) bindings for [Kubernetes](https://kubernetes.io) API objects, and use them to build a [Web Assembly](https://webassembly.org/) (WASM). You could use such a WASM to transform an object or extract some status from it, and plug it into a generic webhook or controller.

## Building and Using the WASM

To build a WASM and some Javascript glue code to light it up you need a Kubernetes cluster and the `kubectl` command line:

```
$ kubctl get all
NAME                 TYPE        CLUSTER-IP   EXTERNAL-IP   PORT(S)   AGE
service/kubernetes   ClusterIP   10.96.0.1    <none>        443/TCP   5d3h
```

and Rust including Cargo and the [`wasm-pack`](https://github.com/rustwasm/wasm-pack) tool. You also need the [OpenAPI Tools CLI](https://github.com/OpenAPITools/openapi-generator), which we download and run using [Jbang](https://www.jbang.dev/). If you have all those installed (e.g. by using the `shell.nix`) then you can just run `make` (ignore warnings) and the build artifacts drop into `./pkg`:

```
$ make
mkdir -p openapi
kubectl get --raw /openapi/v2 | \
        jq 'with_entries(select([.key] | inside(["definitions", "components", "info", "swagger", "openapi"]))) + {paths:{}}' \
        > openapi/k8s.json
jbang org.openapitools:openapi-generator-cli:6.0.0-beta generate -g rust -o openapi -i openapi/k8s.json
...
wasm-pack build --target web
[INFO]: Checking for the Wasm target...
[INFO]: Compiling to Wasm...
   Compiling openapi-rust v0.1.0 (/home/dsyer/dev/scratch/openapi-rust)
warning: function is never used: `set_panic_hook`
 --> src/utils.rs:1:8
  |
1 | pub fn set_panic_hook() {
  |        ^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `openapi-rust` (lib) generated 1 warning
    Finished release [optimized] target(s) in 8.80s
[WARN]: :-) origin crate has no README
[INFO]: Installing wasm-bindgen...
[INFO]: Optional fields missing from Cargo.toml: 'description', 'repository', and 'license'. These are not necessary, but recommended
[INFO]: :-) Done in 9.18s
[INFO]: :-) Your wasm pkg is ready to publish at /home/dsyer/dev/scratch/openapi-rust/pkg.
[WARN]: :-) [35] SSL connect error (error:1404B418:SSL routines:ST_CONNECT:tlsv1 alert unknown ca)
```

The Rust function that is exported from the WASM looks like this (in `src/lib.rs`):

```Rust
#[wasm_bindgen]
pub fn xform(json: &str) -> String {
    let mut deployment: IoK8sApiAppsV1Deployment = serde_json::from_str(json).unwrap();
    // ...
    deployment.spec = Some(spec(deployment.spec, app));
    return serde_json::to_string(&deployment).unwrap();
}
```

It accepts a JSON string, converts it to a Kubernetes `Deployment` and then modifies it, filling in missing fields, eventually to return it back as a different JSON.

The WASM is built to be run in a browser. There is a `bundle.js` wrapper that lets you use the code from [Node.js](https://nodejs.org) and provides a convenient wrapper for the `xform` function in the WASM, so you can work with Javascript objects instead of JSON strings. It is entirely boilerplate:

```javascript
import init, { xform } from "./pkg/openapi_rust.js";
const xform_object = arg => JSON.parse(xform(JSON.stringify(arg)));
const bytes = fs.readFileSync(path.dirname(import.meta.url).replace('file://', '') + '/pkg/openapi_rust_bg.wasm');
let wasm = await init(bytes);

export { wasm, xform_object as xform };
export default wasm;
```

Here's a REPL session where we use the WASM to populate a `Deployment` from an empty input:

```javascript
> var {xform} = await import('./bundle.js')
> xform({})
{
  apiVersion: 'apps/v1',
  kind: 'Deployment',
  metadata: { labels: { app: 'demo' } },
  spec: {
    selector: { matchLabels: [Object] },
    template: { metadata: [Object], spec: [Object] }
  }
}
```

You can reset everything with `make clean`.

## The Build Steps

The `Makefile` tells you what it is doing as it builds. These are the main steps:

1. Extract the schema from the Kubernetes API server objects:

        $ kubectl get --raw /openapi/v2 | jq 'with_entries(select([.key] | inside(["definitions", "components", "info", "swagger", "openapi"]))) + {paths:{}}' > k8s.json
2. Generate the Rust language bindings (including any CRDs):

        $ jbang org.openapitools:openapi-generator-cli:6.0.0-beta generate -g rust -i k8s.json
3. Compile to WASM:

        $ wasm-pack build --target web

## Calling WASM from Rust

Set up dependencies:

```
$ cd runtime
$ cargo install cargo-edit
$ cargo add wasmtime
```

Write a `main.rs` that ships a string into the WASM function and extracts the result:

```Rust
use std::error;
use std::convert::TryInto;
use std::str;
use wasmtime::*;

fn main() -> Result<(), Box<dyn error::Error>> {

let engine = Engine::default();
    let module = Module::new(&engine, include_bytes!("../../wasm/pkg/openapi_rust_bg.wasm"))?;
    let mut store = Store::new(&engine, {});
    let import_object = [
        Func::wrap(&mut store, |_: i32, _: i32| {}).into()
    ];
    let instance = Instance::new(&mut store, &module, &import_object)?;
    let memory = instance.get_memory(&mut store, "memory").unwrap();

    let xform = instance.get_typed_func(&mut store, "xform")?;
    let input = "{}";
    for (i, c) in input.bytes().enumerate() {
        memory.data_mut(&mut store)[8 + i] = c;
    };
    xform.call(&mut store, (0, 8, input.len() as i32))?;
    let ptr = u32::from_le_bytes(memory.data(&mut store)[0..4].try_into().unwrap()) as usize;
    let len = u32::from_le_bytes(memory.data(&mut store)[4..8].try_into().unwrap()) as usize;
    println!("{}", str::from_utf8(&memory.data(&mut store)[ptr..ptr+len]).unwrap());

    Ok(())
}
```

NOTE: The `xform` function is defined as `&str -> String` in Rust, but it comes out as `(ret: i32, ptr: i32, len: i32) -> ()` in the WASM. The result is encoded as a struct of `{*char,size_t}` at the pointer `ret`.

Run it:

```
$ cargo run | jq
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/openapi-rust-runtime`
{
  "apiVersion": "apps/v1",
  "kind": "Deployment",
  "metadata": {
    "labels": {
      "app": "demo"
    }
  },
  "spec": {
    ...
  }
}
```