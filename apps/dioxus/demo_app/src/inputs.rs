//! A simple example of a RuDI interactive table.

// imports
use dioxus::prelude::*;
use rudi_apps::prelude::*;

/// The `UserInputs` app-step component.
#[component]
pub fn UserInputs() -> Element {
    let this = RudiElement::app_step::<()>("user_inputs");
    use_context_provider(|| Namespace::from(&this));
    // rsx! {
    //     div { "{this.namespace.0}" }
    // }
    let integer_value = use_signal(|| 10_i32);
    rsx! {
        div { id: this.id.clone(),
            FluidPage { max_width: Some("800px".to_string()),
                FluidRow {
                    FluidSpan { n_columns: 6,
                        EchoChamber::<i32> { value: integer_value,
                            p { "Integer value from 1 to 100:" }
                            NumericInput::<i32> {
                                name: "integer".to_string(),
                                value: integer_value,
                                min: Some(1),
                                max: Some(100),
                            }
                        }
                    }
                    div {}
                }
            }
        }
    }
}

/// Echo each input's current value to screen.
#[component]
pub fn EchoChamber<T>(value: ReadSignal<T>, children: Element) -> Element 
where T: 'static + std::fmt::Display
{
    rsx! {
        div { {children} }
        div { "Value: {value}" }
    }
}
