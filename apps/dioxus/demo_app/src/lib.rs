//! A simple demonstration of how to build a RuDI app.
//! 
//! This `lib.rs` file defines the app's Dioxus app step components,
//! which must be re-exported here to be found by the app framework.
//! 
//! Typically, you should create one module per app step in its
//! own file or folder.

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
