//! A simple example of a RuDI interactive table.

// imports
use dioxus::prelude::*;
use rudi_apps::prelude::*;

/// The `DemoTable` app-step component.
#[component]
pub fn DemoTable() -> Element {
    let _server_state = use_context::<ServerState>();
    rsx! {
        "DemoTable pending"
    }
}
