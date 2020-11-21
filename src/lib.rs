extern crate html5ever;
extern crate markup5ever_rcdom as rcdom;

use std::default::Default;

use html5ever::driver::ParseOpts;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::{parse_document, serialize};
use rcdom::{RcDom, SerializableHandle};

use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn test(doc: &str) -> String {
    console::log_1(&"Hello using web-sys".into());

    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };
    
    let dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut doc.as_bytes())
        .unwrap();

    let document: SerializableHandle = dom.document.clone().into();
    let mut out = Vec::new();

    serialize(&mut out, &document, Default::default())
        .ok()
        .expect("serialization failed");

    return String::from_utf8(out).unwrap();
}

#[wasm_bindgen]
pub fn square(x: u32) -> u32 {
    x * x
}
