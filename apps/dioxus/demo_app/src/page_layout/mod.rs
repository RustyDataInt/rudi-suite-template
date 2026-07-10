//! A high-level description of the Bootstrap/R Shiny
//! style fluid grid layout.

// imports
use dioxus::prelude::*;
use rudi_apps::prelude::*;

/// The `PageLayout` app-step component.
#[component]
pub fn PageLayout() -> Element {

    // Create a named RuDI element for this app step.
    let this = RudiElement::app_step::<()>("page_layout");

    // Call the `AppStepPage` component to create a standardized app step page.
    rsx! {
        AppStepPage { app_step: this,
            FluidRow {
                p {
                    "Things to do on this page:"
                    ul { margin_top: "5px",
                        li {
                            "Use the descriptions in each panel to see how column widths translate to the on-screen layout."
                        }
                        li {
                            "Resize your browser window from very wide and tall to narrow and short to see what happens."
                        }
                        li { "Click the help icon above for more information." }
                    }
                }
            }
            FluidRow {
                DataPanel { name: "panel_a", title: "Panel A", n_columns: 6,
                    div {
                        "Row 1, Span 1 - "
                        "n_columns: 6 (out of 12)"
                    }
                }
                DataPanel { name: "panel_b", title: "Panel B", n_columns: 6,
                    div {
                        "Row 1, Span 2 - "
                        "n_columns: 6 (out of 12)"
                    }
                    div { "with" }
                    div { "additional" }
                    div { "lines of content" }
                }
            }
            FluidRow {
                DataPanel { name: "panel_c", title: "Panel C", n_columns: 3,
                    div {
                        "Row 2, Span 1 - "
                        "n_columns: 3 (out of 12)"
                    }
                }
                DataPanel { name: "panel_d", title: "Panel D", n_columns: 2,
                    div {
                        "Row 2, Span 2 - "
                        "n_columns: 2 (out of 12)"
                    }
                }
                DataPanel { name: "panel_e", title: "Panel E", n_columns: 5,
                    div {
                        "Row 2, Span 3 - "
                        "n_columns: 5 (out of 12)"
                    }
                }
            }
        }
    }
}
