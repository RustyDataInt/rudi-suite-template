## User Inputs

The User Inputs demo page shows various types of user inputs
you can use in your app to make it interactive.

### Source code

The 
[exact code that builds this page](https://github.com/RustyDataInt/rudi-suite-template/blob/main/apps/dioxus/demo_app/src/user_inputs/mod.rs)
is provided in the 
[Rudi Suite Template](https://github.com/RustyDataInt/rudi-suite-template), 
making it easy to "learn by copying".

### Implementation strategy

User inputs in RuDI apps are an extension of standard 
[Dioxus Components](https://dioxuslabs.com/components)
to make them stateful so that they can be saved in bookmarks (see below).

Stateful behavior is achieved through a RuDI-specific structure called 
a `RudiElement`. For most user inputs, you don't need to create your own 
RudiElements, you can just use the input components built into the RuDI
apps framework, e.g., this code creates a stateful numeric input.

```rust
const NAME: &str = "input_name";
let value = use_signal(|| this.get_initial_state(NAME, 0_i32));
NumericInput::<i32> {
    name: NAME.to_string(),
    value,
    label: "Input Label".to_string(),
}
```

### Saving bookmarks

An important feature of working with RuDI apps is that you
can save "bookmarks" of the current state of the app, including
the page you are on and the working values of all user inputs.

When you work, **save as many bookmarks as often as you want**!
Example uses include:
- sharing your app state with someone else
- saving bookmarks for each figure in your manuscript for later revision
