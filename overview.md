---
title: "Tool Suite Template"
has_children: false
nav_order: 0
---
<!--- edit the title above with the short name of your repository, 
      e.g, "My Tools", which will appear on the menu tab item -->

<!-- please do not alter the next line -->
{% include rudi-project-overview.md %}

<!-- replace this section with markdown content describing your tool suite -->
<!-- https://www.markdownguide.org/basic-syntax/ -->

These pages provide a detailed description of the **RuDI tool suite template**, 
which you can use to create your own tool suite of pipeline and apps. 

- <https://github.com/RustyDataInt/rudi-suite-template>

### Quick start

[**Click here**](https://github.com/RustyDataInt/rudi-suite-template/generate) 
to create a new tool suite repository from the template.

You will be prompted for the user and name of the repository you would like 
to create.

Open and edit the following files, using the instructions in comments
to find lines to edit to match your needs:

- _config.yml
- overview.md
- LICENSE
- README.md

Then copy and modify the '_template' pipeline or app, which provides a working 
boilerplate for all required code. 

### Further documentation

The rest of these pages walk you through
the basic structure of pipeline and app assemblies and 
provide a working reference as you write code.

In addition, you will want to explore the documentation for the
pipelines and apps frameworks that provide support functions
for writing tools:

- [pipelines framework](/rudi-pipelines-framework)
- [apps framework](/rudi-apps-framework)

<!-- please do not alter the next line -->
{% include rudi-project-documentation.md %}
