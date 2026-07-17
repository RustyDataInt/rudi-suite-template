//! A demonstration of a RuDI interactive plot.

// imports
use dioxus::prelude::*;
use rudi_apps::prelude::*;
use super::*;

/// The `DataPlots` app-step component.
#[component]
pub fn DataPlots() -> Element {

    // Create the named RuDI element for this app step.
    let this = RudiElement::app_step::<()>("data_plots");

    // Load the demo data as `Resource<DataFrame>`.
    let data_frame = use_resource(move || async move {
        ServerData::load_global("iris", load_iris).expect("Failed to load iris data")
    });
    
    // Configure the plot display.
    let config = use_signal(|| {
        PlotConfig::<f64, f64>::builder()
            .title("Demo Plot (Iris Data)")
            .series_with_defaults("sepal_width", "sepal_length", SeriesType::Points)
            .series_with_defaults("sepal_width", "petal_length", SeriesType::Points)
            .x_label("Sepal Width")
            .y_label("Sepal/Petal Length")
            // .x_range(0.0..6.0)
            // .y_range(0..6)
    });

    // Use the `AppStepPage` component to create a standardized app step page.
    rsx! {
        AppStepPage { app_step: this,
            FluidRow {
                PlotPanel::<(),f64,f64> {
                    name: "plot_panel".to_string(),
                    title: "Plot Panel".to_string(),
                    n_columns: 4,
                    data_frame: Some(data_frame),
                    config,
                }
            }
        }
    }
}
