//! A simple demonstration of how to build a RuDI app.
//! 
//! This `lib.rs` file defines the app's Dioxus app step components,
//! which must be re-exported here to be found by the app framework.
//! 
//! Typically, you should create one module per app step.

// app-step modules
mod demo_plot;
mod demo_table;

// re-export the app step components defined in the app-step modules
pub use demo_plot::DemoPlot;
pub use demo_table::DemoTable;
