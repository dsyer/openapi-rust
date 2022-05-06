use std::error;

use wasmtime::*;

fn main() -> Result<(), Box<dyn error::Error>> {
    let engine = Engine::default();
    let module = Module::new(&engine, include_bytes!("../../wasm/pkg/openapi_rust_bg.wasm"))?;
    let mut store = Store::new(&engine, {});
    // The module doesn't import anything, so we create an empty import object.
    let import_object = [];
    let instance = Instance::new(&mut store, &module, &import_object)?;
    let memory = instance.get_memory(&mut store, "memory").unwrap();

    let xform = instance.get_typed_func(&mut store, "xform")?;
    let input = "{}";
    for (i, c) in input.bytes().enumerate() {
        memory.data_mut(&mut store)[i] = c;
    };
    let result = xform.call(&mut store, (0, input.len() as i32))?;
    println!("Result {:?}", result);

    Ok(())
}
