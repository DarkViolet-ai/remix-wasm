import init from "../../rust-wasm-lib/pkg/rust_wasm_lib_bg.wasm?init";

export async function loadWasm() {
  await init();
}
