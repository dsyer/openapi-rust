import init, { xform } from "./pkg/image.js";
const bytes = fs.readFileSync(path.dirname(import.meta.url).replace('file://', '') + '/pkg/image_bg.wasm');
let wasm = await init(bytes);

export { wasm, xform };
export default wasm;