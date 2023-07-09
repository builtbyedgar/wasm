use wasm_bindgen::prelude::*;

mod fib;
mod shared;
// mod store;

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let _body = document.body().expect("document should have a body");
    let root = document
        .get_element_by_id("root")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();

    // Manufacture the element we're gonna append
    // let val = document.create_element("h2")?;
    // val.set_text_content(Some("Hello from Rust! ⚡️"));

    // root.append_child(&val)?;

    let val = document
        .query_selector("h2")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    val.set_inner_text(&"⚡️ Hello from Rust!");

    Ok(())
}
