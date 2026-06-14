//! Command-line tools for processing __TOOL_SUITE_NAME__ data.

// dependencies
use std::env;
use std::error::Error;

// modules
mod tools;

// constants
const TOOLS_NAME: &str = "xxx_tools";

// load and process data
fn main() -> Result<(), Box<dyn Error>> {

    // read command line arguments
    let mut args: Vec<String> = env::args().collect();
    args = args[1..].to_vec(); // drop executable name 
    if args.len() == 0 { // check for something to do, i.e., a tool to run
        eprintln!("{}: missing tool or command", TOOLS_NAME);
        Err(format!("usage: {} <tool> [additional arguments]", TOOLS_NAME))?
    }
    let tool = args[0].clone(); // drop tool name
    // args = args[1..].to_vec(); // uncomment if passing additional arguments to tools

    // dispatch to tool or command
    match tool.as_str() {

        /*--------------------------------------------------------------
        <pipeline> <action>
        ------------------------------------------------------------- */
        // tool description here
        "tool_name" => tools::tool_name::stream(),

        // etc.

        /*--------------------------------------------------------------
        unrecognized pipeline action tool
        ------------------------------------------------------------- */
        _ => Err(format!("{}: unknown tool or command: {}", TOOLS_NAME, tool))?
    }
}
