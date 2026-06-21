//! A simple example of a RuDI interactive table.

// imports
use dioxus::prelude::*;
use rudi_suite::prelude::*;

/// The `DemoTable` app-step component.
#[component]
fn DemoTable() -> Element {
    let server_state = use_context::<ServerState>();
    rsx! {
        "DemoTable pending"
    }
}
