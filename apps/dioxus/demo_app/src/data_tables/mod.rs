//! A demonstration of a RuDI interactive table.

// imports
use dioxus::prelude::*;
use rudi_apps::prelude::*;
use rlike::data_frame::prelude::*;

/// The `DataTables` app-step component.
#[component]
pub fn DataTables() -> Element {

    // Create the named RuDI element for this app step.
    let this = RudiElement::app_step::<()>("page_layout");

    // Generate mock data.
    let x = vec![1, 2, 3, 4, 5];
    let mut y = x.clone();
    y.reverse();
    let z = x.iter().map(|v| v * 2).collect::<Vec<i32>>();
    let df = use_signal(|| df_new!{
        col_x = x.to_rl(),
        col_y = y.to_rl(),
        col_z = z.to_rl(),
    });
    let config = TableConfig {
        columns: None,
        sort_columns: None,
        sort_ascending: None,
        max_rows: 10,
    };

    // Use the `AppStepPage` component to create a standardized app step page.
    rsx! {
        AppStepPage { app_step: this,
            FluidRow {
                TablePanel::<()> {
                    name: "table_panel".to_string(),
                    title: "Table Panel".to_string(),
                    n_columns: 4,
                    data_frame: Some(df),
                    config,
                }
            }
        }
    }
}
