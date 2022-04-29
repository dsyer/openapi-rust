globalThis.alert = (...args) => {
    console.log(...args);
}

import init, { greet } from "./pkg/openapi_rust.js";
const bytes = fs.readFileSync(path.dirname(import.meta.url).replace('file://', '') + '/pkg/openapi_rust_bg.wasm');
let wasm = await init(bytes);

export { wasm, greet };
export default greet;