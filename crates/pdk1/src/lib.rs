use extism_pdk::*;
use serde::Serialize;

const VOWELS: &[char] = &['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'];

#[derive(Serialize)]
struct TestOutput {
    pub count: i32,
    pub config: String,
    pub a: String,
}

#[plugin_fn]
pub fn count_vowels(input: String) -> FnResult<Json<TestOutput>> {
    let mut count = 0;
    for ch in input.chars() {
        if VOWELS.contains(&ch) {
            count += 1;
        }
    }

    set_var!("a", "this is var a")?;

    let a: Vec<u8> = var::get("a")?.expect("variable 'a' set");
    let a = String::from_utf8(a).expect("string from varible value");
    let config = config::get("thing").expect("'thing' key set in config");

    let output = TestOutput { count, config, a };
    Ok(Json(output))
}

use wasmi::*;

#[derive(Serialize)]
pub struct Example(String);

#[plugin_fn]
pub fn outer_wasm(_: ()) -> FnResult<Json<Example>> {

    let engine = Engine::default();
    let wasm = std::fs::read("target/wasm32-wasi/debug/pdk2.wasm")?;
    let module = Module::new(&engine, &mut &wasm[..])?;

//    dbg!(&module.exports());

    type HostState = i32;

    let mut store = Store::new(&engine, 42);
    let fn_type = FuncType::new([], []);
    let print = Func::new(&mut store, fn_type, |_caller_, _param, _result| {
        println!("println ssss");
        Ok(())
    });

    let mut linker = <Linker<HostState>>::new(&engine);
    linker.define("env", "print", print)?;
    let instance = linker
        .instantiate(&mut store, &module)?
    //     ;
    // let instance = instance
        .start(&mut store)?
        ;
    let hello = instance.get_typed_func::<(), i32>(&store, "hello")?;

    let x = hello.call(&mut store, ())?;
    //println!("hello: {x}");

    let output = Example(format!("hello: {x}"));
    Ok(Json(output))
}