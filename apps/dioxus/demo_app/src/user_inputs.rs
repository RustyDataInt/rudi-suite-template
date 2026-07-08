//! A display of the various RuDI inputs with a simple screen echo.

// imports
use dioxus::prelude::*;
use rudi_apps::prelude::*;

/// The `UserInputs` app-step component.
#[component]
pub fn UserInputs() -> Element {

    // Create a named RuDI element for this app step.
    let this = RudiElement::app_step::<()>("user_inputs");

    // Define signals for all child components that are inputs. Inherit 
    // starting signal values from the initial server state in case the user  
    // launched the app from a bookmark, otherwise the default values are used.
    const INTEGER: &str = "integer";
    let integer = use_signal(|| this.get_initial_state(INTEGER, 10_i32));
    const FRACTIONAL: &str = "fractional";
    let fractional = use_signal(|| this.get_initial_state(FRACTIONAL, 2.5_f64));
    const STRING: &str = "string";
    let string = use_signal(|| this.get_initial_state(STRING, "some text".to_string()));

    // Call the `AppStepPage` component to create a standardized app step page.
    rsx! {
        AppStepPage { app_step: this, max_width: Some("800px".to_string()),
            FluidRow {
                FluidSpan { n_columns: 6,
                    EchoChamber::<i32> { value: integer,

                        // Use RuDI stateful input components to build your app.
                        NumericInput::<i32> {
                            name: INTEGER.to_string(), // yields CSS id 'app-user_inputs-integer'
                            value: integer,
                            label: "i32 from 2 to 20, step 2".to_string(),
                            min: Some(2),
                            max: Some(20),
                            step: Some(2),
                        }
                    }
                    EchoChamber::<f64> { value: fractional,
                        NumericInput::<f64> {
                            name: FRACTIONAL.to_string(),
                            value: fractional,
                            label: "f64 from 2.0 to 3.0, step 0.01".to_string(),
                            min: Some(2.0),
                            max: Some(3.0),
                            step: Some(0.01),
                        }
                    }
                    EchoChamber::<String> { value: string,
                        TextInput::<String> {
                            name: STRING.to_string(),
                            value: string,
                            label: "String input".to_string(),
                            placeholder: "type some text...".to_string(),
                        }
                    }
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
        div { style: "background-color: white; border: 1px solid #444; padding: 5px; margin: 5px; border-radius: 5px;",
            div { {children} }
            div { style: "margin: 10px;", "Value: {value}" }
        }
    }
}
