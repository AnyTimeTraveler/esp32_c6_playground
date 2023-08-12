#[no_mangle]
pub extern "C" fn fmodf(a: f32, b: f32) -> f32 {
    libm::fmodf(a, b)
}

#[no_mangle]
pub extern "C" fn fmod(a: f64, b: f64) -> f64 {
    libm::fmod(a, b)
}

#[no_mangle]
pub extern "C" fn fminf(a: f32, b: f32) -> f32 {
    libm::fminf(a, b)
}

#[no_mangle]
pub extern "C" fn fmin(a: f64, b: f64) -> f64 {
    libm::fmin(a, b)
}

#[no_mangle]
pub extern "C" fn fmaxf(a: f32, b: f32) -> f32 {
    libm::fmaxf(a, b)
}

#[no_mangle]
pub extern "C" fn fmax(a: f64, b: f64) -> f64 {
    libm::fmax(a, b)
}


use esp_println::println;
use wasmi::{Caller, Engine, Func, Linker, Module, Store};


pub(crate) fn run_wasm_example() {
    // First step is to create the Wasm execution engine with some config.
    // In this example we are using the default configuration.
    let engine = Engine::default();
    // let wat = r#"
    //     (module
    //         (import "host" "hello" (func $host_hello (param i32)))
    //         (func (export "hello")
    //             (call $host_hello (i32.const 3))
    //         )
    //     )
    // "#;
    // Wasmi does not yet support parsing `.wat` so we have to convert
    // out `.wat` into `.wasm` before we compile and validate it.
    // let wasm = wat::parse_str(&wat)?;
    let wasm = [0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x60, 0x01, 0x7F, 0x00, 0x60, 0x00, 0x00, 0x02, 0x0E, 0x01, 0x04, 0x68, 0x6F, 0x73, 0x74, 0x05, 0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x00, 0x00, 0x03, 0x02, 0x01, 0x01, 0x07, 0x09, 0x01, 0x05, 0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x00, 0x01, 0x0A, 0x08, 0x01, 0x06, 0x00, 0x41, 0x03, 0x10, 0x00, 0x0B, 0x00, 0x14, 0x04, 0x6E, 0x61, 0x6D, 0x65, 0x01, 0x0D, 0x01, 0x00, 0x0A, 0x68, 0x6F, 0x73, 0x74, 0x5F, 0x68, 0x65, 0x6C, 0x6C, 0x6F];

    // let wasm = Vec::new();
    let module = Module::new(&engine, &wasm[..]).unwrap();

    // All Wasm objects operate within the context of a `Store`.
    // Each `Store` has a type parameter to store host-specific data,
    // which in this case we are using `42` for.
    type HostState = u32;
    let mut store = Store::new(&engine, 42);
    let host_hello = Func::wrap(&mut store, |caller: Caller<'_, HostState>, param: i32| {
        println!("Got {param} from WebAssembly");
        println!("My host state is: {}", caller.data());
    });

    // In order to create Wasm module instances and link their imports
    // and exports we require a `Linker`.
    let mut linker = <Linker<HostState>>::new(&engine);
    // Instantiation of a Wasm module requires defining its imports and then
    // afterwards we can fetch exports by name, as well as asserting the
    // type signature of the function with `get_typed_func`.
    //
    // Also before using an instance created this way we need to start it.
    linker.define("host", "hello", host_hello).unwrap();
    let instance = linker
        .instantiate(&mut store, &module).unwrap()
        .start(&mut store).unwrap();
    let hello = instance.get_typed_func::<(), ()>(&store, "hello").unwrap();

    // And finally we can call the wasm!
    hello.call(&mut store, ()).unwrap();
}