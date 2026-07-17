//! A demonstration of a RuDI interactive table.

// imports
use dioxus::prelude::*;
use rudi_apps::prelude::*;
use super::*;

/// The `DataTables` app-step component.
#[component]
pub fn DataTables() -> Element {

    // Create the named RuDI element for this app step.
    let this = RudiElement::app_step::<()>("data_tables");

    // Load the demo data as `Resource<DataFrame>`.
    let data_frame = use_resource(move || async move {
        ServerData::load_global("iris", load_iris).expect("Failed to load iris data")
    });
    
    // Configure the table display.
    let config = use_signal(|| {
        TableConfig::new(TableSelectMode::Single)
            // .selectable(TableSelectMode::Single)
            // .searchable()
            // .paginate(20)
    });

    // Use the `AppStepPage` component to create a standardized app step page.
    rsx! {
        AppStepPage { app_step: this,
            FluidRow {
                TablePanel::<()> {
                    name: "table_panel".to_string(),
                    title: "Table Panel".to_string(),
                    n_columns: 4,
                    data_frame: Some(data_frame),
                    config,
                }
            }
        }
    }
}
