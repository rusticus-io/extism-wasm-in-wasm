


```console

cargo build -p pdk1 --target wasm32-wasi

cargo build -p pdk2 --target wasm32-wasi

extism call target/wasm32-wasi/debug/pdk1.wasm outer_wasm --allow-path target/wasm32-wasi/debug:target/wasm32-wasi/debug

```