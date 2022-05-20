use std::error;
use std::convert::TryInto;
use std::str;
use std::env;

use wasmtime::*;

fn main() -> Result<(), Box<dyn error::Error>> {

    let args: Vec<String> = env::args().collect();
    let input: &str = if args.len()>1 {
        &args[1]
    } else {
        "{}"
    };

    let engine = Engine::default();
    // let module = Module::new(&engine, include_bytes!("../../wasm/target/wasm32-unknown-unknown/release/openapi_rust.wasm"))?;
    let module = Module::new(&engine, include_bytes!("../../image/pkg/image_bg.wasm"))?;
    let mut store = Store::new(&engine, {});
    let import_object = [];
    let instance = Instance::new(&mut store, &module, &import_object)?;
    let memory = instance.get_memory(&mut store, "memory").unwrap();

    let xform = instance.get_typed_func(&mut store, "xform")?;
    for (i, c) in input.bytes().enumerate() {
        memory.data_mut(&mut store)[8 + i] = c;
    };
    xform.call(&mut store, (0, 8, input.len() as i32))?;
    let ptr = u32::from_le_bytes(memory.data(&mut store)[0..4].try_into().unwrap()) as usize;
    let len = u32::from_le_bytes(memory.data(&mut store)[4..8].try_into().unwrap()) as usize;

    println!("{}", str::from_utf8(&memory.data(&mut store)[ptr..ptr+len]).unwrap());

    Ok(())
}
