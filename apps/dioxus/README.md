---
title: App-Specific Files
parent: Apps
has_children: false
nav_order: 10
published: true
---

## {{page.title}}

A RuDI **app** is a web application that creates an interactive
interface for analyzing data packages created by pipelines. RuDI uses 
[Dioxus](https://dioxuslabs.com/)
to create "fullstack" apps that can be run on many different 
platforms, including public web servers, remote HPC servers, 
and desktop or laptop computers.

## TLDR: Initializing a new suite and app

Edit files:
- .../apps/dioxus/suite_config.toml (once per tool suite)
- .../apps/dioxus/Cargo.toml (add each new app dependency)
- .../apps/dioxus/shared/server/Cargo.toml (add each new app member)
- .../apps/dioxus/<your_app_library> (source code for each new app)
- .../apps/dioxus/shared/<your_shared_library> (source code for suite-level functions)
- .../apps/dioxus/shared/server/assets/* (CSS, Javacript, or other suite-level assets)

## Multi-app Rust workspace organization

RudI tool suites are structured to carry zero, one, or multiple 
apps in a multi-app configuration.

### Workspace Cargo.toml file with app members

The `Cargo.toml` file in the root of this `apps` folder sets  
up the Rust workspace for your multi-app tool suite. Edit it  
to list one workspace member for each app folder. Retain this 
configuration even if you only have one app.

```toml
# your_tool_suite/apps/dioxus/Cargo.toml
members = [
    "shared/server", # do not delete
    "my_app1",       # same as your app folder name
    "my_app2"        # etc.
]
```

### Shared folder for code relevant to multiple apps

The `shared` folder has one subfolder called `server`, which 
is a Rust binary crate with the `main.rs` file and `main()` 
function that runs your apps. You must also edit its `Cargo.toml` 
file to declare a dependency on each of your app crates. 
Otherwise, most developers do not need to edit this 
framework-level crate. 

```toml
# your_tool_suite/apps/dioxus/shared/server/Cargo.toml
[dependencies]
my_app1 = { path = "../../my_app1" } # matching the app workspace members
my_app2 = { path = "../../my_app2" } # always use the ../.. path prefix
```

The `shared` folder is also the place to put any 
tool-suite-specific library crates that define your Dioxus components,
utilities, and server functions shared by more than one app.

### One apps folder for each app

Each additional subfolder in the `apps` folder is a Rust 
library crate that defines a single Dioxus app, i.e., with 
a `lib.rs` file. Edit this crate to describe the app and to 
create app-specific components, utilities, and server functions
(see below).

## Suite-level configuration file

Following Rust and Dioxus style, the apps framework uses TOML
configuration files for describing your tool suite and apps.

First, edit `suite_config.toml` in this apps root folder,
which defines metadata for your tool suite and sets operating
parameters that apply to all apps.

```toml
# your_tool_suite/apps/dioxus/suite_config.toml
name = "your_tool_suite" # must match the app's Cargo.toml
label = "Your Suite Label"
description = "Short text description of your tool suite."

# plus additional fields found in the template file
```

## Building a new app

Create a folder for your app in the app folder.

###




At the root level of the app, these files include
(see comments within for additional instructions):

- **config.yml**  = names the app and describes its structure
- **server.R**    = contains the function 'appServer'
- **overview.md** = text used to describe the app on its splash page

The name of the app's folder must match the name provided in config.yml.

Optionally, you can organize additional app scripts into the
following sub-folders, which will be loaded along
with _config.yml_ and _server.R_ when the app loads
(see other pages for a description of files in those folders):

- modules
- types
- ui
- utilities 

## App versioning

### App versions

Individual app versioning is optional but recommended as it will
be displayed in the web page UI and help users
access legacy versions of your code to analyze their data according
to some previous standard.

To track app versions, add a semantic version
key to config.yml and update it prior to committing new app code. 
It is not necessary to create Git tags for app versions.

```yml
# shiny/apps/<app>/config.yml
name: Example
description: "Example of descriptive text"
version: v0.0.0
```

### External suite versions

If your app uses code modules from other tool suites, you may
wish to specify the required versions of those external suites.
This is useful if you don't wish to adjust your app to account for a
breaking change made in an external tool suite.  Declare such version
requirements as follows, replacing 'suiteName' with the name of the
external tool suite.

```yml
# shiny/apps/<app>/config.yml
name: Example
description: "Example of descriptive text"
version: v0.0.0
suiteVersions: # <<< add this section <<<
    suiteName: v0.0.0
```

If you do not provide a version for an external tool suite,
the latest version will be used.

If you only use app code from within your own tool suite, the 
suiteVersions dictionary can be omitted.
