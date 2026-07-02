//! A simple example of a RuDI interactive plot.

// imports
use dioxus::prelude::*;
use rudi_apps::prelude::*;

/// The `DemoPlot` app-step component.
#[component]
pub fn DemoPlot() -> Element {
    let _server_state = use_context::<ServerState>();
    rsx! {
        "DemoPlot pending"
    }
}
