use wasm_bindgen::prelude::*;
mod utils;
mod query;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(ss: &str) {
    let txt = format!("Hello, {}", ss);
    alert(&txt);
}

#[wasm_bindgen]
pub fn add2(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn queryAttr(html: &str, selecter: &str, attr: &str) -> Box<[JsValue]> {
    utils::set_panic_hook();
    let list = query::query_attr(html, selecter, attr);
    let iter = list.iter().map(|e| JsValue::from_str(e));    
    iter.collect::<Vec<JsValue>>().into_boxed_slice()
}