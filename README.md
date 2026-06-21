# RuDI Tools Suite Template

The [Rusty Data Interface](https://rustydataint.github.io/) (RuDI) 
is a standardized framework for developing and running HPC data 
analysis **pipelines** and interactive visualization **apps**
with a Rust-first mindset.

This is a **repository template** for creating a new **RuDI tools suite**. 
Follow the instructions below to copy this repository, then fill your 
copy with code to define a suite of your own data analysis tools.

## Template usage

### Create a new repository from this template

**To get started quickly**, 
[click here to create a new suite repository from this template](https://github.com/RustyDataInt/rudi-suite-template/generate).

You will be prompted for the user and name of the repository you would like 
to create.

### Copy and use the demo pipeline or app

The easiest way to start a new tool is to rename and modify the `demo_pipeline`
or `demo_app`, which provide working boilerplate for all required code. The
demos are fully functional, so you can use them to practice running a simple
pipeline or app before modifying them. If you don't rename the demos, you'll 
eventually want to delete them.

### Additional usage instructions

The following documentation pages have detailed information on the features 
of your new tool suite and the RuDI frameworks that will help you quickly 
develop pipelines and apps:

- [tool suite template documentation](https://RustyDataInt.github.io/rudi-suite-template)
- [pipelines framework documentation](https://RustyDataInt.github.io/rudi-pipelines-framework)
- [apps framework documentation](https://RustyDataInt.github.io/rudi-apps-framework)

### Finishing your tool suite README.md

This file becomes the README.md for your tool suite.
Delete everything up to this section, then modify the sections below 
to match your tool suite. 

Most tool suites are recommended to use a single-suite intallation.
If a multi-suite installation is better for your application, or not 
recommended at all, modify the different installation sections as needed.

Instructions below are written assuming you will support containers.
Modify those sections below if you only support local conda environments.

# __TOOL_SUITE_NAME__ Tool Suite

**__TOOL_SUITE_NAME__** carries pipelines and apps for
DOING SOMETHING INTERESTING.

The steps to using __TOOL_SUITE_NAME__ are to:
- install the codebase in this repository
- obtain or build the required runtime environment
- MORE STEPS HERE
- visualize results in the interactive apps

## Single-suite installation (recommended)

__TOOL_SUITE_NAME__ is implemented in the
[Rusty Data Interface](https://RustyDataInt.github.io/) (RuDI)
for developing, installing and running HPC **pipelines** 
and interactive web **apps** in a standardized design interface
with a Rust-first mindset. For __TOOL_SUITE_NAME__, 
we recommend a single-suite installation, which is accomplished by:
- cloning this tool suite repository
- running _install.sh_
- optionally running _alias.pl_ to create an `__TOOL_SUITE_SHORT_NAME__` alias to the command line interface (CLI)

See the bottom of this README for information on an alternative 
container-only way of using of __TOOL_SUITE_NAME__ pipelines.

### Install this tool suite

Run the following to install the tool suite and command line interface (CLI).

```bash
git clone https://github.com/__GIT_OWNER__/__TOOL_SUITE_NAME__.git
cd __TOOL_SUITE_NAME__
./install.sh
```

### Create an alias to the command line interface (optional)

```bash
# you can use a different alias if you'd like, e.g., replace __TOOL_SUITE_SHORT_NAME__ with __TOOL_SUITE_NAME__
perl alias.pl __TOOL_SUITE_SHORT_NAME__ 
```

Answer 'y' to add the alias to your bash profile, then 
reload a new shell to activate the alias for use.

If you prefer not to use an alias, 
add the installation directory to your PATH variable
or `cd` into the directory prior to calling `./__TOOL_SUITE_SHORT_NAME__`.

### Test the command line interface (CLI)

For help, call the CLI with no arguments, which describes the format for pipeline calls. 

```sh
# use the alias, if you created it as described above
__TOOL_SUITE_SHORT_NAME__ 
__TOOL_SUITE_SHORT_NAME__ --help

# or call the CLI directly without an alias
cd __TOOL_SUITE_NAME__
./__TOOL_SUITE_SHORT_NAME__
./__TOOL_SUITE_SHORT_NAME__ --help
```

## Obtain or build the required runtime environment

__TOOL_SUITE_NAME__ pipelines use version-controlled 3rd-party software built into a 
[conda](https://docs.conda.io/)
runtime environment. There are two ways to obtain or create that environment.

### Use Singularity containers, i.e., Apptainers (recommended)

__TOOL_SUITE_NAME__ supports Singularity containers that have the required
conda environments pre-installed. No action is needed to support their use
if either the `singularity` or `module load singularity` command is available 
on your server - the containers will download and be used automatically.

If you wish to use containers but need to run a different command
to make `singularity` available, follow the instructions in
`.../rudi/config/singularity.yml` to communicate that command to the CLI.

### Build the Conda environments locally

If you don't have Singularity or Apptainer available on your system,
or prefer or need to build the environments yourself, you can
build them in your __TOOL_SUITE_NAME__ installation as follows:

```sh
__TOOL_SUITE_SHORT_NAME__ <pipeline> conda --create
```

Note that internally conda environments are built using
[micromamba](https://mamba.readthedocs.io/).

In a shared server environment, the environment build command may get killed by the host.
If that happens, run the command on a cluster worker node with sufficient resources, e.g.:

```sh
# example for a Slurm-based cluster server
salloc --account <your_slurm_account> --cpus-per-task 4 --mem-per-cpu 4G 
__TOOL_SUITE_SHORT_NAME__ <pipeline> conda --create
exit
```

### Communicate your environment choice using option `--runtime`

Option `--runtime` defaults to value 'auto', which will prefer to use
Singularity containers and fall back to locally built environments only if 
that fails. To force the use of a specific environment, set the 
option to either `--runtime singularity` or `--runtime conda`.

## Execute a pipeline from the command line

### Job files

__TOOL_SUITE_NAME__ pipelines can be called entirely using the CLI introduced above. However, you 
are encouraged to create YAML-format job configuration files that define the
parameters for your job and execution steps.

See [the templates folder](https://github.com/__GIT_OWNER__/__TOOL_SUITE_NAME__/tree/main/templates)
for job file templates for all __TOOL_SUITE_NAME__ pipelines
and actions, and <https://RustyDataInt.github.io/rudi/docs/job_config_files.html>
for extended help on using job files. Job file templates can also be generated with 
command `__TOOL_SUITE_SHORT_NAME__ <pipeline> template`, e.g., `__TOOL_SUITE_SHORT_NAME__ analyze template`.

The __TOOL_SUITE_NAME__ CLI and job files can run pipeline actions either 
inline in the calling shell or by submitting jobs to your server job scheduler.
The latter is recommended for most use cases. Thus, our most common usage pattern is:

```sh
__TOOL_SUITE_SHORT_NAME__ inspect myJob.yml          # check the formatting of your job file
__TOOL_SUITE_SHORT_NAME__ mkdir myJob.yml            # create any missing output directories
__TOOL_SUITE_SHORT_NAME__ submit --dry-run myJob.yml # test the job file to see what will happen
__TOOL_SUITE_SHORT_NAME__ submit myJob.yml           # submit the job to Slurm or your scheduler
__TOOL_SUITE_SHORT_NAME__ myJob.yml status           # show the state of all submitted jobs
__TOOL_SUITE_SHORT_NAME__ myJob.yml top              # monitor a running job
__TOOL_SUITE_SHORT_NAME__ myJob.yml report           # show a job log report
__TOOL_SUITE_SHORT_NAME__ myJob.yml ls               # show the contents of a job's output diretory
```

### Workflow sequence

__TOOL_SUITE_NAME__ has _pipelines_ each with associated _actions_, 
listed here in execution order of the most common actions:
- `__PIPELINE__ __ACTION__` (where `__PIPELINE__` is a _pipeline_ and `__ACTION__` is an _action_)

Required/common options are described below; use 
`__TOOL_SUITE_SHORT_NAME__ <pipeline> <action> --help` or `__TOOL_SUITE_SHORT_NAME__ <pipeline> template` 
for complete option information, or see the action help 
[here](https://github.com/__GIT_OWNER__/__TOOL_SUITE_NAME__/tree/main/options).

### Universally required options

Options `--output-dir/-O` and `--data-name/-N` are required by all pipeline actions.
Output files are placed into directory `<--output-dir>/<--data-name>`.

## Required input files

COMPLETE THIS SECTION.

## Recommended __TOOL_SUITE_NAME__ workspace organization

The following is an optional but time-tested strategy for organizing input,
output, job, and resource files in your __TOOL_SUITE_NAME__ workspace 
(create *** folders manually as needed).

```sh
__TOOL_SUITE_NAME__        # root folder you created above
├── input                  # *** folder for your input data files
│   └── project1           # *** subfolder for a related set of samples
├── jobFiles               # *** folder for your job configuration files
│   └── project1
├── rudi                   # RuDI codebase folder created by __TOOL_SUITE_NAME__ installation
│   ├── config             # folder with configuration files you may need
│   └── resources          # folder where resource files are placed by default
└── output                 # *** folder for your output data files
    └── project1
```

## ADDITIONAL DESCRIPITION SECTIONS

COMPLETE AS NEEDED.

## Launch the interactive apps server

Once all pipelines have finished running, you will want to view and interact
with your data.

To install and launch the __TOOL_SUITE_NAME__ apps server, we recommend using the 
[RuDI Desktop](https://RustyDataInt.github.io/rudi-desktop-app),
which allows you to control both local and remote app servers.

After following the instructions to run the Desktop on your local machine
or server, load a __TOOL_SUITE_NAME__ data package file ending in `.rudi.package.zip`,
into the app interface. If possible, the relevant container
will automatically be downloaded and used to run the apps server.

## Alternative multi-suite installation

You can alternatively install __TOOL_SUITE_NAME__ as part of a RuDI
**multi-suite installation** that carries multiple distinct tool suites.

In the multi-suite mode, you will:
- clone and install the RuDI framework
- add __TOOL_SUITE_NAME__ (and later others) to your installation
- call the `rudi` utility to use tools from any installed suite

### Install the RuDI framework

Run the following to install the RuDI command line interface (CLI).

```bash
git clone https://github.com/RustyDataInt/rudi.git
cd rudi
./install.sh
```

### Add a rudi alias to .bashrc (optional)

```bash
./rudi alias --help
./rudi alias --alias rudi # change the alias name if you'd like 
rudi
```

Answer 'y' to add the alias to your bash profile, then 
reload a new shell to activate the alias for use.

If you prefer not to use an alias, 
add the RuDI installation directory to your PATH variable,
or `cd` into the directory prior to calling `./rudi`.

### Add __TOOL_SUITE_NAME__ to your installation

```bash
./rudi add --help
./rudi add --suite __GIT_OWNER__/__TOOL_SUITE_NAME__
```

Later you can add additional tool suites to this same RuDI installation 
as needed, which can reduce disk utilization from duplicated files.

## Alternative container-only method of using __TOOL_SUITE_NAME__ pipelines

Some users may only be interested in using __TOOL_SUITE_NAME__ pipelines
as a standalone program, e.g., if __TOOL_SUITE_NAME__ actions are to be
incorporated into a pre-existing workflow managment system.
__TOOL_SUITE_NAME__ pipeline containers support this installation-free usage.

Importantly, running pipelines via a direct call to a container,
rather than via the CLI installed on your server, disables
all of the worflow/job manager helpers in favor of your 
pre-exiting solution. All data processing tasks and configuration 
remain exactly the same since the container has the same installation
in it as described above.

### Download the relevant container

You do not need to clone or install this repository, simply download the relevant 
container image from:
- <https://github.com/__GIT_OWNER__/__TOOL_SUITE_NAME__/pkgs/container/__LOWER_CASE_TOOL_SUITE_NAME__> 

e.g., using command:
- `singularity pull oras://ghcr.io/__GIT_OWNER__/__LOWER_CASE_TOOL_SUITE_NAME__:v0.0`

### Use the container with directory bind mounts and direct action calls

As always, you must bind mount all relevant server paths to 
the container so that running pipeline jobs have access to your files. The 
recommended directory organization above makes this easy by requiring only one bind
mount of the __TOOL_SUITE_NAME__ root directory (even that is unnecessary
if you work from that directory as the current working directory is implicitly
mounted).

Then simply follow `singularity run <image>.sif` with  your pipeline, action, 
and options as you would for any typical data analysis program, e.g.:

```sh
# every relevant directory option should be prefixed with a bind mount directory
ROOT_DIR=/path/to/__TOOL_SUITE_NAME__
singularity run \
    --bind $ROOT_DIR \
    <image>.sif \
        <pipeline> <action> \
            --tmp-dir        $ROOT_DIR/tmp \
            --output-dir     $ROOT_DIR/output \
            --data-name      my-data-name \
            --option-name    123
            # etc.
```

The job report log is printed to STDOUT for you to consume as needed in your workflow.
