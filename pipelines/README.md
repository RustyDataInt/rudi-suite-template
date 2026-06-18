---
title: Pipelines
has_children: true
nav_order: 20
published: true
---

## {{page.title}}

A **pipeline** (also called a workflow) is a single, coordinated set
of data analysis instructions that can be called by name using the
command line utility, e.g.,

```bash
rudi <pipeline> ...
```

A pipeline is distinct from many standalone programs in
that a pipeline might have many inputs and outputs and typically makes 
calls to many installed programs. Thus, rather than being a program itself, 
a pipeline is a means of coordinating a reproducible set of calls to 
other programs.
