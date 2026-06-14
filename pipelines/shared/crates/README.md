---
published: false
---

## Rust crates for compiled binaries

The `shared/crates` folder is the best place to put any 
[Rust](https://rust-lang.org/)
code to be compiled into __TOOL_SUITE_NAME__ binary executables,
where Rust is the recommended language for developing new
HPC data analysis tools that are extremely fast and memory safe.

As described below, RuDI framework utilities make it easy
to compile, test, and distribute your Rust binaries.

If you are new to Rust, a great place to get started is the outstanding
[Rust Programming Language](https://doc.rust-lang.org/book/) book.

### A single common xxx_tools utility dispatch (recommended)

A common and recommended approach is for a tool suite to create a 
single `xxx_tools` binary that provides a dispatch to multiple required
utilities via one crate/binary to minimize overhead and simplify 
distribution. A crate following this pattern has been initialized 
in this folder. You need to add code for your specific tools and 
register them in the dispatcher. 

### Compiling Rust binaries during development

Developers will need to compile Rust code using the support features 
provided by the rudi-pipelines-framework. The following steps create
a Conda environment that provides the Rust cargo compiler and
then compile code, with options to load system C compiler support
if needed by your crates.

```bash
# must compile from within the crate directory
cd /path/to/__TOOL_SUITE_NAME__/rudi/suites/developer-forks/__TOOL_SUITE_NAME__/shared/crates/xxx_tools 
rudi analyze rust --help
rudi analyze rust --create  1.92 # create a versioned Rust development environment
rudi analyze rust --compile 1.92 # compile xxx_tools using the created environment
rudi analyze rust --gcc "module load gcc/15.1.0" --compile 1.92 # if a command is required to make C compilers available 
```

It is also possible to compile the Rust code from first principles
if all prerequisites are met. The compiled executable binary must be 
copied into file `rudi/bin/__TOOL_SUITE_NAME__/dev/xxx_tools`.

Use of the developer binary in the `dev` folder is activated using
the `rudi -d` developer option on all pipeline calls.

### Distributing compiled binaries via GitHub Actions

Most users do not want to compile code, which can be frought with complications. 
Instead, copy file `templates/continuous_integration/compile_rust_with_c.yml`
into folder `.github/workflows` at the base of your tool suite repository.
Change just one or two variables in it and Continuous Integration (CI) via 
[GitHub Actions](https://github.com/features/actions) will automatically 
attach compiled x86_64 Linux binaries to versioned code releases on GitHub
whenever the major or minor (but not the patch) version changes.

### Calling binaries in your action scripts

You can then write one or two lines in your pipeline code to download the 
binary automatically on the users behalf and then use it:

```bash
# my_action_script.sh

# SUITE_NAME is always set in a running pipeline
# GITHUB_OWNER and BINARY_NAME you define in your code
getVersionedBinary ${GITHUB_OWNER}/${SUITE_NAME} ${BINARY_NAME}

# VERSIONED_BINARY_PATH is set by the getVersionedBinary utility
# you may prefer to use an alias to VERSIONED_BINARY_PATH
export MY_TOOLS_BIN=${VERSIONED_BINARY_PATH} 

${MY_TOOLS_BIN} tool_name ...
```

It's that easy!
