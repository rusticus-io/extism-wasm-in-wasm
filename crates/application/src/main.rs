use std::collections::BTreeMap;

use extism::*;

fn main() -> anyhow::Result<()> {
    let wasm = std::fs::read("target/wasm32-wasi/debug/pdk1.wasm")?;
    let ctx = Context::new();
    let mut plugin = Plugin::new(&ctx, &wasm, [], true)?;
    let mut config = BTreeMap::new();
    config.insert("thing".to_string(), Some("thing".to_string()));
    plugin.set_config(&config)?;
    let data = plugin.call("count_vowels", "this is a test")?;
    println!("{}", String::from_utf8(data.to_vec())?);
    assert_eq!(data, b"{\"count\":4,\"config\":\"thing\",\"a\":\"this is var a\"}");
    Ok(())
}