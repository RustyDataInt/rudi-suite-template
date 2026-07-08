## RuDI app src folder contents

Your RuDI app folder `<app_name>/src` must contain
a `lib.rs` file that re-exports one module per app step.

```rust
// declare the app-step modules
mod app_step_name;

// re-export the app step components defined in the app-step modules
pub use app_step_name::AppStepComponent;
```

You may optionally include file `instructions.md` to
provide detailed instruction for an app step that users
can show by clicking the matching icon link.
