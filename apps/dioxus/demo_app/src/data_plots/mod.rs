//! A demonstration of a RuDI interactive plot.

// imports
use dioxus::prelude::*;
use rudi_apps::prelude::*;
use rlike::data_frame::prelude::*;

/// The `DataPlots` app-step component.
#[component]
pub fn DataPlots() -> Element {

    // Create the named RuDI element for this app step.
    let this = RudiElement::app_step::<()>("data_plots");

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

    let config = use_signal(|| {
        PlotConfig::<i32, i32>::builder()
            .series_with_defaults("col_x", "col_y")
            .series_with_defaults("col_x", "col_z")
            .x_label("Column X")
            .y_label("Column Y or Z")
            .y_range(0, 6)
    });

    // Use the `AppStepPage` component to create a standardized app step page.
    rsx! {
        AppStepPage { app_step: this,
            FluidRow {
                PlotPanel::<(),i32,i32> {
                    name: "plot_panel".to_string(),
                    title: "Plot Panel".to_string(),
                    n_columns: 4,
                    data_frame: Some(df),
                    config,
                }
            }
        }
    }
}
