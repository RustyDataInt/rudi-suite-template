
# RuDI Demo app

Replace the contents of this file with a brief descriptive overview 
of your app, in one to perhaps several short paragraphs.

See the
<a target="exteral" href="https://www.markdownguide.org/basic-syntax/" >
    markdown syntax guide
</a>
for instructions on how to use basic markdown formatting
such as **emphasis**.

## Plots and tables

At the heart of most data visualization app are interactive
plots and tables. 
This app demonstrates the core plot and table components at
the heart of most data visualization apps.

`RudiPlot` and `RudiTable` take `Vec<T>`, i.e., a vector
of instances of a struct of type T as input.

`RudiDataFramePlot` and `RudiDataFrameTable` take and
`rlike::DataFrame` as input.

### Plots

Regardless of the data inputs, the plot components put 
a dynamic plot image into a RudiCard.

### Tables

The table components similar create a dynamic data table
in a RudiCard, which support row filtering, ordering,
and selection.
