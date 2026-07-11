## Page Layout

The Page Layout demo page uses a few panels to illustrate
how a "fluid page" layout works, as adopted from
[Boostrap](https://getbootstrap.com/)
and
[R Shiny](https://shiny.posit.co/).

### Source code

The 
[exact code that builds this page](https://github.com/RustyDataInt/rudi-suite-template/blob/main/apps/dioxus/demo_app/src/page_layout/mod.rs)
is provided in the 
[Rudi Suite Template](https://github.com/RustyDataInt/rudi-suite-template)
used to create a new tool suite, making it easy to 
"learn by copying".

### Implementation strategy

The `FluidPage` layout consists of a series of `FluidRows`,
each of which has **12 vertical columns**. A `FluidRow` contains
a series of `FluidSpans` - which in RuDI apps are nearly always 
`DisplayPanels` - with fixed, declared widths from 1 to 12 columns each.

Thus, you can have one span or column with a width of 12 that 
fills an entire row, or put multiple panels in a row.

`FluidRows` - and thus the panels they contain -  expand to fill 
the available page width, unless you set property `max_width` on 
the parent `FluidPage` or `AppStepPage`, in which case they may
only fill the left part of a page.

There is usually a `min_width` set on the individual spans and 
panels, such that if a user makes a very narrow browser window,
the panels will begin to cascade in a vertical stack.
