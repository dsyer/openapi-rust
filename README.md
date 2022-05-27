This project shows how to generate [Rust](https://www.rust-lang.org/) bindings for [Kubernetes](https://kubernetes.io) API objects, and use them to build a [Web Assembly](https://webassembly.org/) (WASM). You could use such a WASM to transform an object or extract some status from it, and plug it into a generic webhook or controller. There are two WASM modules to build:

* "wasm": transforms a Kubernetes Deployment resource, adding labels and selectors, making it "valid".
* "image": takes a CRD representing a container image, and queries the repository for its latest sha256.

## The Deployment Transformer

To build a WASM and some Javascript glue code to light it up you need a Kubernetes cluster and the `kubectl` command line:

```
$ kubctl get all
NAME                 TYPE        CLUSTER-IP   EXTERNAL-IP   PORT(S)   AGE
service/kubernetes   ClusterIP   10.96.0.1    <none>        443/TCP   5d3h
```

and Rust including Cargo and the [`wasm-pack`](https://github.com/rustwasm/wasm-pack) tool. If you have all those installed (e.g. by using the `shell.nix`) then you can just run `make` (ignore warnings) and the build artifacts drop into `./pkg`:

```
$ cd wasm
$ make
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
    let mut deployment: Deployment = serde_json::from_str(json).unwrap();
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

Here's a REPL session where we use the WASM to populate a `Deployment` from an empty input. You need to tell Node.js that the `pkg` bundle is a module first:

```
$ cat pkg/package.json | jq '. + {type:"module"}' > pkg/tmp.json && mv pkg/tmp.json pkg/package.json
```

then

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

## The Image Transformer

This WASM takes an image resource and calculates a status for it that has the latest sha256 from the repository. It imports an async function `getWithHeaders()` that does the work of sending an HTTP GET and returning the result as an object. The translation of the result into WASM memory is handled by `wasm-bindgen` so it only really works with a JavaScript runtime.

```
$ cd image
$ make
```

The imported `getWithHeaders()` is defined like this:

```rust
#[wasm_bindgen(module = "runtime")]
extern "C" {
    async fn getWithHeaders(url: &str, headers_json: &str) -> JsValue;
}
```

which creates a dependency in JavaScript on library called "runtime", so we have to add that to `package.json`:

```json
{
  "type": "module",
  "dependencies": {
    "runtime": "file:./runtime"
  }
}
```

and `npm install` it. The "runtime" library is just a simple wrapper around the `http` builtin from Node.js:

```javascript
import http from 'http';

function get(url, headers) {
	return new Promise((resolve, reject) => {
		http.get(url, { "headers": headers }, response => {
			response.setEncoding('utf8');
			let data = '';
			response.on('data', (chunk) => {
				data += chunk.toString();
			});
			response.on('end', () => {
				resolve({data: data, headers: response.headers, status: response.statusCode});
			});
			response.on('error', (error) => {
				reject(error);
			});
		});
	});
}

export async function getWithHeaders(url, headers) {
	var result = await get(url, headers);
	return result;
}
```

With that library in place we can run the WASM from the command line:

```
$ node
> var {xform} = await import('./bundle.js')
undefined
> await xform({spec:{image:"localhost:5000/apps/demo"}})
{
  complete: true,
  latestImage: 'localhost:5000/apps/demo@sha256:95c043ec7f3c9d5688b4e834a42ad41b936559984f4630323eaf726824a803fa'
}
```

Since we are stuck with a JavaScript runtime, we don't have to bother serializing and deserializing the objects into strings as they pass in and out of the WASM (`wasm-bindgen` does that for us). So the Rust code for the WASM is a little different in that the functions can just accept and return `JsValue`:

```rust
#[wasm_bindgen(module = "runtime")]
extern "C" {
    async fn getWithHeaders(url: &str, headers_json: &str) -> JsValue;
    fn log(value: String);
}

#[wasm_bindgen]
pub async fn xform(json: JsValue) -> JsValue {
    let image: V1Image = json.into_serde().unwrap();
    let mut status = V1ImageStatus::new();
    // calculate status...
    return JsValue::from_serde(&status).unwrap();
}
```

## Kubernetes Controller

There is a Kubernetes controller that uses the image transformer from the last section to update the status of an image resource. It is written in Node.js using the `@kubernetes/client-node` library. That way we can simply `import` the `xform` function from the `bundle.js` in the "image" module and call it to calculate the image status.

Set up the CRD:

```
$ kubectl apply -f image.yaml
```

Run the controller:

```
$ node main.js
5/26/2022, 11:07:47 AM: Watching API
```

Add an image resource and delete it:

```
$ kubectl apply -f demo.yaml
```

and:

```
5/26/2022, 11:07:47 AM: Received event in phase ADDED.
5/26/2022, 11:07:48 AM: Reconciling demo
```

Modify the resource and apply it again, then delete it and watch the controller logs:

```
5/26/2022, 11:11:20 AM: Received event in phase MODIFIED.
5/26/2022, 11:11:21 AM: Reconciling demo
5/26/2022, 11:17:53 AM: Received event in phase DELETED.
5/26/2022, 11:17:53 AM: Deleted demo
```

and see the result:

```
$ kubectl get images
NAME   IMAGE                      LATEST
demo   localhost:5000/apps/demo   localhost:5000/apps/demo@sha256:95c043ec7f3c9d5688b4e834a42ad41b936559984f4630323eaf726824a803fa
```

## Calling WASM from Rust

Set up dependencies:

```
$ cd runtime
$ cargo install cargo-edit
$ cargo add wasmtime
```

There is a `main.rs` that ships a string into the WASM function and extracts the result.

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

or add an argument to the command line to pass in a different starting point:

```
$ cargo run '{"metadata":{"labels":{"app":"foo"}}}' | jq
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/openapi-rust-runtime '{"metadata":{"labels":{"app":"foo"}}}'`
{
  "apiVersion": "apps/v1",
  "kind": "Deployment",
  "metadata": {
    "labels": {
      "app": "foo"
    }
  },
  "spec": {
    "selector": {
      "matchLabels": {
        "app": "foo"
      }
    },
    ...
  }
}
```