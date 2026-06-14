//! Tool description here.

// modules
// mod tool_name_module;

// dependencies
use std::error::Error;
use rustc_hash::FxHashMap;
use crossbeam::channel::{bounded, unbounded};
use rayon::prelude::*;
use rudi::pub_key_constants;
use rudi::workflow::{Workflow, Config, Counters};

// constants for environment variable, config, and counter keys, etc.
const TOOL: &str = "tool_name";
pub_key_constants!(
    // from environment variables
    N_CPU
    INPUT_FILE // etc.
    // counter keys
    N_LINES
    N_LINES_BY_GROUP // etc.
);

// function called by xxx_tools main()
pub fn main() -> Result<(), Box<dyn Error>> {

    // get config from environment variables
    let mut cfg = Config::new();
    cfg.set_usize_env( &[N_CPU]);
    cfg.set_string_env(&[INPUT_FILE]); // etc.

    // initialize counters
    let mut ctrs = Counters::new(TOOL, &[
        (N_LINES, "lines in input file"),
    ]);
    ctrs.add_keyed_counters(&[
        (N_LINES_BY_GROUP,    "number of lines by XXX group"),
    ]);

    // initialize the tool
    let mut w = Workflow::new(TOOL, cfg, ctrs);
    w.log.initializing();

    // continue to do work here
    // use parallel processing when possible

    // print counts
    w.ctrs.print_grouped(&[
        &[N_LINES],
        &[N_LINES_BY_GROUP],
    ]);
    Ok(())
}
