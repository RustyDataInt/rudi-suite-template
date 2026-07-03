---
title: Singularity Containers
has_children: false
nav_order: 50
published: true # set to false to remove this tab from your suite's doc site
---

> In 2022, the Apptainer project was released as an open-source fork
> of Singularity, but Apptainer installations still support the `singularity` 
> command. For simplicity and historical reasons, MDI documentation and code  
> mainly use the Singularity name rather than Apptainer.

## {{page.title}}

Developers can help users enjoy the fastest and most controlled pipeline
and app execution by supporting Singularity containers, i.e. Apptainers.
You can choose to wrap your entire tool suite, or just individual pipelines, in 
container images that you distribute in a registry, such as the recommended 
GitHub Container Registry.

- Singularity: <https://sylabs.io/guides/latest/user-guide/>
- Apptainer: <https://apptainer.org/>
- GitHub Container Registry: <https://docs.github.com/en/packages/>

In all cases, the user's system must support Singularity containers. 
If it does not, the MDI CLI will revert to conda-based "direct" execution
even if you have distributed containers as described below.

### Suite-level containers

The simplest approach is to enable your entire tool suite for container support
by editing files (see comments within for more information):

- _config.yml
- singularity.def

There are two types of suite-level containers. Stage 1 containers run
pipelines using conda environments built in the container, whereas Stage 2
containers, suffixed with "-apps", run a Shiny apps server using R
packages installed in the container.

The advantage of this approach is its simplicity. A potential disadvantage is 
the larger size of the resulting Stage 1 container if your pipelines must build  
several different conda environments.

An example of the relevant section of _\_config.yml_ activated for suite-level containers is:

```yml
# _config.yml
container:
    supported:  true 
    registry:   ghcr.io 
    owner:      GITHUB_OWNER 
    installer:  apt-get
    stages:
        pipelines: true 
        apps:      false
```

### Pipeline-level containers

Alternatively, you may place individual Stage 1 pipelines into their own containers.
This is accomplished by appropriate edits to:

- pipeline.yml
- pipelines/\<pipeline\>/singularity.def

This approach is advantageous when your tool suite has multiple different pipelines,
each with its own unique conda enviroment, where breaking them into separate
containers allows users to download ony the smaller container(s) they need.

Note that suite-level containers take precedence, so set `container:supported` 
or `container:stages:pipelines` to `false` in _\_config.yml_ if you support 
pipeline-level containers.

An example of the relevant section of pipeline.yml activated for pipeline-level containers is:

```yml
# pipeline.yml
container:
    supported: true 
    registry:  ghcr.io 
    owner:     GITHUB_OWNER 
    installer: apt-get  
```

### Container configuration via singularity.def

The operating system and system libraries to be made available in a Stage 1
pipelines container are specified in _singularity.def_, while program dependencies are 
provided by conda environments pre-installed into the container. In other words,
containers still rely on proper `condaFamilies` declarations - what differs is where
the conda environments are built and by whom.

A complete description of Singularity definition files is beyond scope here, 
but most developers can simply use _singularity.def_ as it is provided in the 
suite template.  Otherwise, you might think about changing:

```yml
# singularity.def

# declare the operating system
Bootstrap: docker
From: ubuntu:24.04

# add to the %post scriptlet to prepare your container in ways beyond conda
%post
```

Stage 2 apps suite-level containers always use the same pre-defined Singularity
definition file. You influence what is installed into the container by 
declaring R package dependencies in your app configuration files.

### Where pipeline code comes from when using versioned containers

Importantly, MDI containers do not carry the final version of all
pipeline code found in a tool suite, e.g., your action scripts.
Those script files are always taken from the code in the 
user's tool suite installation and can reflect a patch version more
recent than the suite version that was used to build a container.

What MDI containers provide is the proper program environment, including 
built conda environments, that are needed to run your pipeline scripts. 
Thus, containers will change much less frequently than your pipeline actions. 
Containers only need to change when a program dependency must be added or updated.

As described in more detail 
[here](https://RustyDataInt.github.io/rudi-suite-template/docs/suite_versions.html),
the MDI convention is that any change to the required working environment
of your code is reflected in a change to either the major or minor version,
but that patch version changes never require a new program dependency and
thus do not trigger a new container build. For this reason, MDI container
images are only tagged with major.minor, but not patch versions.

### Building container images via Continuous Integration with GitHub Actions

The conventions above make it easy to support automated container image
building via Continuous Integration (CI) with 
[GitHub Actions](https://github.com/features/actions).

Specifically, the
[MDI Tool Suite Template](https://RustyDataInt.github.io/rudi-suite-template/overview)
you should use to create your new tool suite provides templates for 
CI workflows. Copy them as needed from the `templates` to the `.github/workflows`
folder of your tools suite, change just one or two variables as documented within, and
the relevant container images will be built and be made available for automatic download  
by end users whenever you push a new `major.minor.0` version tag to GitHub. 

It's that easy!

### Bypassing containers during code development

Container images are the tool to use for stable distribution environments.
You will still likely want and need to build conda environments manually while 
developing code and exploring different program dependency needs.

In the early development, simply leave the container flags above set to
`supported: false`, which will require the use of locally built conda environments.
Use this 'conda-only' mode until you have a stable environment configuration.

Once you have released initial Stage 1 containers, running in developer mode, i.e.,
with the `mdi -d` flag set, will default to using the container from the latest
version tag found in the definitive suite repository. If you need to modify 
and test an updated conda environment, set `--runtime conda` to bypass any existing 
containers and force the use of the working conda environment as you rebuild it 
using `mdi <pipeline> conda --create`.
