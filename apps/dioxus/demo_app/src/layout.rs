//! A simple example of a RuDI interactive table.

// imports
use dioxus::prelude::*;
use rudi_apps::prelude::*;

/// The `DemoTable` app-step component.
#[component]
pub fn PageLayout() -> Element {
    rsx! {
        div { "pending" }
    }
    // let this = RudiElement::new::<()>("page_layout");
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
