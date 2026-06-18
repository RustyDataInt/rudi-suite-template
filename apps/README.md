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
and desktop and laptop computers.

### Multi-app Rust workspace organization

The `Cargo.toml` file in the root of this `apps` folder
sets up the Rust workspace for your multi-app tool suite.
Edit it to list one workspace member for each app folder.
Retain this configuration even if you only have one app.

The `shared` folder has one subfolder called `server`, 
which is a Rust binary crate with the `main.rs` file and 
`main()` function that runs your apps. You must also edit
its `Cargo.toml` file to declare a dependency on each of 
your app crates where indicated. Otherwise, most developers 
do not need to edit this framework-level crate. 

The `shared` folder is also the place to put any
tool-suite-specific library crates that define Dioxus components,
utilities, and server functions shared by more than one app.

Each additional subfolder in the `apps` folder is a
Rust library crate that defines a single Dioxus app,
i.e., with a `lib.rs` file. Edit this crate to describe 
the app and to create app-specific components, utilities,
and server functions.



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

## Stage 2 versioning

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
