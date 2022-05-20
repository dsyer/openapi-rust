import init, { xform } from "./pkg/image.js";
const xform_object = arg => JSON.parse(xform(JSON.stringify(arg)));
const bytes = fs.readFileSync(path.dirname(import.meta.url).replace('file://', '') + '/pkg/image_bg.wasm');
let wasm = await init(bytes);

export { wasm, xform_object as xform };
export default wasm;