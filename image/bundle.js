import init, { xform } from "./pkg/image.js";
import fs from "fs";
import path from "path";
const xform_object = arg => { if (!arg || typeof(arg)!='object') {arg = {}}; return xform(arg); }
const bytes = fs.readFileSync(path.dirname(import.meta.url).replace('file://', '') + '/pkg/image_bg.wasm');
let wasm = await init(bytes);

export { wasm, xform_object as xform };
export default wasm;