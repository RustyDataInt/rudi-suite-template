//! A demonstration of various RuDI inputs with a simple screen echo.

// imports
use dioxus::prelude::*;
use rudi_apps::prelude::*;

// declare input names
const FRACTIONAL: &str = "fractional";
const STRING:     &str = "string";
const BOOLEAN:    &str = "boolean";
const SELECT:     &str = "select";
const RADIO_GROUP: &str = "radio_group";

/// The `UserInputs` app-step component.
#[component]
pub fn UserInputs() -> Element {

    // Create the named RuDI element for this app step.
    let this = RudiElement::app_step::<()>("user_inputs");

    // Define value signals for all child inputs. Inherit starting signal values 
    // from the initial server state in case the user launched the app from a 
    // bookmark, otherwise the default values are used.

    let (i32_name, i32_signal) = use_numeric_input(&this, "i32", 10_i32);

    let fractional = use_signal(|| this.get_initial_state(FRACTIONAL, 2.5_f64));
    let string = use_signal(|| this.get_initial_state(STRING, "some text".to_string()));
    let boolean = use_signal(|| this.get_initial_state(BOOLEAN, true));
    let select = use_signal(|| this.get_initial_state(SELECT, 2_u8));
    let radio_group = use_signal(|| this.get_initial_state(RADIO_GROUP, "rad_one".to_string()));

    // Use the `AppStepPage` component to create a standardized app step page.
    rsx! {
        AppStepPage { app_step: this, max_width: Some("800px".to_string()),
            FluidRow {
                p {
                    "Things to do:"
                    ul { margin_top: "5px",
                        li { "Update the input values and watch your changes echo to the screen." }
                        li { "Click the input panel header bar to expand/collapse it." }
                        li {
                            "Click 'Save Your Work! - to your computer' to the left to save a bookmark of your selections."
                        }
                        li {
                            "After reloading the browser, load your bookmark to re-launch the app into the same saved state."
                        }
                        li { "Click the help icon above for more information." }
                    }
                }
            }
            FluidRow {
                InputPanel { name: "inputs", n_columns: 12,
                    InputRow {
                        NumericInput::<f64> {
                            name: FRACTIONAL.to_string(),
                            value: fractional,
                            label: "Any f64".to_string(),
                        }
                        NumericInput::<i32> {
                            name: i32_name.clone(), // yields CSS id 'app-user_inputs-i32'
                            value: i32_signal,
                            label: "i32, 2 to 20, step 2".to_string(),
                            min: Some(2),
                            max: Some(20),
                            step: Some(2),
                        }
                        BooleanSwitch {
                            name: BOOLEAN.to_string(),
                            value: boolean,
                            label: "Boolean".to_string(),
                        }
                        TextInput {
                            name: STRING.to_string(),
                            value: string,
                            label: "String input".to_string(),
                            placeholder: "Type some text...".to_string(),
                        }
                    }
                    InputRow {
                        SelectInput {
                            name: SELECT.to_string(),
                            value: select,
                            label: "Named u8 options".to_string(),
                            choices: vec![1, 2, 3, 4, 5],
                            labels: Some(
                                vec![
                                    "option one".to_string(),
                                    "option two".to_string(),
                                    "option three".to_string(),
                                    "option four".to_string(),
                                    "option five".to_string(),
                                ],
                            ),
                        }
                        RadioGroupInput {
                            name: RADIO_GROUP.to_string(),
                            value: radio_group,
                            label: "Radio Group".to_string(),
                            choices: vec!["rad_one".to_string(), "rad_two".to_string(), "rad_three".to_string()],
                            labels: Some(vec!["rad 1".to_string(), "rad 2".to_string(), "rad 3".to_string()]),
                        }
                    }
                }
            }
            FluidRow {
                DisplayPanel { name: "input_echo", n_columns: 4,
                    table { style: "background-color: white; border-collapse: collapse; text-align: left;",
                        tr { border_bottom: "1px solid #444",
                            th { "Input" }
                            th { "Echoed Value" }
                        }
                        tr {
                            td { "{FRACTIONAL}" }
                            td { "{fractional}" }
                        }
                        tr {
                            td { "{i32_name}" }
                            td { "{i32_signal}" }
                        }
                        tr {
                            td { "{BOOLEAN}" }
                            td { "{boolean}" }
                        }
                        tr {
                            td { "{STRING}" }
                            td { "{string}" }
                        }
                        tr {
                            td { "{SELECT}" }
                            td { "{select}" }
                        }
                        tr {
                            td { padding_right: "20px", "{RADIO_GROUP}" }
                            td { "{radio_group}" }
                        }
                    }
                }
            }
        }
    }
}
