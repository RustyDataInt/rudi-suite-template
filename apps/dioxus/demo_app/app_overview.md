
# RuDI Demo app

Replace the contents of this file with a brief descriptive overview 
of your app, in one to perhaps several short paragraphs.

See the
<a target="exteral" href="https://www.markdownguide.org/basic-syntax/" >
    markdown syntax guide
</a>
for instructions on how to use basic markdown formatting
such as **emphasis**.

## User inputs

In order to make your app do anything interactive, you will invariably 
want to add inputs for your users to make choices. The first step/tab 
in the demo app shows a library of inputs in a simple action that echoes 
the input value to screen.

## Page layout

RuDI apps use a fluid grid-like component layout that is derived  
from 
<a target="exteral" href="https://getbootstrap.com/">Boostrap</a>
and 
<a target="exteral" href="https://shiny.posit.co/">R Shiny</a>
and time-tested to work simply and well. The second step/tab in the 
demo app walks through the basics of this layout scheme.

## Plots and tables

The thid and fourth steps/tabs of the demo app demonstrate the core 
RuDI plot and table components at the heart of data visualization apps.



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
