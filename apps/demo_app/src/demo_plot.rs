//! A simple example of a RuDI interactive plot.

// imports
use dioxus::prelude::*;
use rudi_suite::prelude::*;

/// The `DemoPlot` app-step component.
#[component]
fn DemoPlot() -> Element {
    let server_state = use_context::<ServerState>();
    rsx! {
        "DemoPlot pending"
    }
}
