//! A simple example of a RuDI interactive plot.

// imports
use dioxus::prelude::*;
// use rudi_apps::prelude::*;

/// The `DemoPlot` app-step component.
#[component]
pub fn DemoPlot() -> Element {
    rsx! {
        div { "pending" }
    }
    // let this = RudiElement::new::<()>("demo_plot");
    // use_context_provider(|| Namespace::from(&this));
    // rsx! {
    //     div { id: this.id,
    //         FluidPage {
    //             FluidRow {
    //                 FluidSpan { n_columns: 6,
    //                     div { "pending" }
    //                 }
    //             }
    //         }
    //     }
    // }
}
