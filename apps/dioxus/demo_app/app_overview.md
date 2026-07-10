
# RuDI Demo app

DEVELOPERS: Replace the contents of this file with a brief description 
of your app, in one to several short paragraphs.

See the
<a target="exteral" href="https://www.markdownguide.org/basic-syntax/" >
    markdown syntax guide
</a>
for instructions on basic markdown formatting.

## User inputs

In order to make your app interactive, you will usually want to add inputs 
for your users to make choices. The first step/tab in the demo app shows 
a library of inputs with a simple echo of input values to the screen.

## Page layout

RuDI apps use a fluid grid-like layout derived from 
<a target="exteral" href="https://getbootstrap.com/">Boostrap</a>
and 
<a target="exteral" href="https://shiny.posit.co/">R Shiny</a>
that is time-tested to work simply and well. The second step/tab in the 
demo app walks through the basics of this layout scheme.

## Plots and tables

The third and fourth steps/tabs of the demo app demonstrate the RuDI 
plot and table components at the heart of data visualization apps.

### Data inputs

For flexibility, `RudiPlot` and `RudiTable` take `Vec<T>`, i.e., 
a vector of instances of type `T` as input. In contrast, 
`RudiDataFramePlot` and `RudiDataFrameTable` take an
`rlike::DataFrame` as input.

### Plots

Regardless of the data input type, the table components
create a dynamic data table in a `DataPanel`, which supports
row filtering, ordering, and selection.

### Tables

The plot components similarly render a dynamic plot image 
into a `DataPanel`.
