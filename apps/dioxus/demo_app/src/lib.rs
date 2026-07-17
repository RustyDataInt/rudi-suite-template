//! A simple demonstration of how to build a RuDI app.
//! 
//! This `lib.rs` file defines the app's Dioxus app step components, which must 
//! be re-exported here to be found by the app framework.
//! 
//! Typically, you should create one module per app step in its own file or folder.

// app-step modules
mod user_inputs;
mod page_layout;
mod data_tables;
mod data_plots;

// re-export the app step components defined in the app-step modules
pub use user_inputs::UserInputs;
pub use page_layout::PageLayout;
pub use data_tables::DataTables;
pub use data_plots::DataPlots;

// imports
use std::error::Error;
use rlike::data_frame::prelude::*;

/// Load the iris CSV file as a global resource for the demo app.
pub fn load_iris(_: &str, _: &str) -> Result<DataFrame, Box<dyn Error>> {
    // "Sepal.Length","Sepal.Width","Petal.Length","Petal.Width","Species"
    // 5.1,3.5,1.4,0.2,"setosa"
    let mut df = df_new!(
        sepal_length: f64, 
        sepal_width:  f64, 
        petal_length: f64,
        petal_width:  f64,
        species:      String
    );
    df_read!(
        &mut df, 
        bytes = include_bytes!("../assets/iris.csv"), 
        header = true, 
        sep = b',', 
        capacity = 200
    );
    Ok(df)
}
