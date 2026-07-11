## User Inputs

The User Inputs demo page shows various types of user inputs
you can build into your app to make it interactive.

### Source code

The 
[exact code that builds this page](https://github.com/RustyDataInt/rudi-suite-template/blob/main/apps/dioxus/demo_app/src/user_inputs/mod.rs)
is provided in the 
[Rudi Suite Template](https://github.com/RustyDataInt/rudi-suite-template)
used to create a new tool suite, making it easy to 
"learn by copying".

### Implementation strategy

User inputs in RuDI apps are an extension of the standard 
[Dioxus Components](https://dioxuslabs.com/components)
to make them stateful so that they can be saved in bookmarks (see below).

Stateful behavior is achieved through a RuDI-specific structure called 
a `RudiElement`. For user inputs, you usually don't create your own 
RudiElements, you instead call the RuDI-specific components built into 
the apps framework, e.g., this code creates a stateful numeric input.

```rust
const INTEGER: &str = "integer";
let integer = use_signal(|| this.get_initial_state(INTEGER, 10_i32));
NumericInput::<i32> {
    name: INTEGER.to_string(),
    value: integer,
    label: "Integer".to_string(),
}
```

### Saving bookmarks

An important feature of working with RuDI apps is that you
can save "bookmarks" of the current state of the app, including
the page you are on and the working values of all user inputs.

When you work, **save as many bookmarks as often as you want**!
Example usages include:
- sharing your app state with someone else
- saving bookmarks for each figure in your manuscript for later revision
