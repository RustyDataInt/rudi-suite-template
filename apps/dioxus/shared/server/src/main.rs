//! The `main.rs` file in the `server` crate is the entry point for the server 
//! application that runs your apps.
//! 
//! You do not usually edit this file.  All code it needs is either:
//!   - built by `server::build.rs` via `rudi_apps::server::build()`
//!   - called by the built code from `rudi_apps::server` and other modules
//!   - a simple, unchanging wrapper that defines the main page layout
//!
//! However, there are advanced cases where you might want to customize the 
//! server to add middleware (see DEVELOPER_NOTE below).

// imports
use dioxus::prelude::*;
use rudi_apps::prelude::*;

// additional imports of app crates, which must re-export all app step components
include!(concat!(env!("OUT_DIR"), "/app_imports.rs"));

// module wrapper for loading the static server configuration
mod server_config {
    use std::sync::OnceLock;
    use rudi_apps::server::*;
    #[allow(dead_code)]
    static SERVER_CONFIG: OnceLock<ServerConfig> = OnceLock::new();

    /// Return a reference to the suite config, with all app configs.
    #[allow(dead_code)]
    pub fn get_server_config() -> &'static ServerConfig {
        SERVER_CONFIG.get_or_init(|| {
            let server_config_toml_str = include_str!(
                concat!(env!("OUT_DIR"), "/server_config.toml")
            );
            toml::from_str(server_config_toml_str)
                .expect("Failed to parse generated ServerConfig TOML")
        })
    }
}

/// This `main()` function is the entry point for the shared backend server.
#[cfg(feature = "server")]
fn main() {
    env_logger::Builder::from_env( // initialize server logging
        env_logger::Env::default().default_filter_or("info")
    ).init();
    let server_config = server_config::get_server_config();
    let max_upload_megabytes = server_config.suite_config.max_upload_megabytes;
    dioxus::serve(|| async move {
        // override DefaultBodyLimit as needed for large file uploads
        use dioxus::fullstack::axum_core::extract::DefaultBodyLimit;
        let max_upload_bytes: usize = max_upload_megabytes as usize * 1024 * 1024;
        let router = dioxus::server::router(RudiServerBoundaries)
            .layer(DefaultBodyLimit::max(max_upload_bytes));

            // DEVELOPER_NOTE: Add any necessary middleware here.

        Ok(router)
    })
}

/// This `main()` function is the entry point for the client WASM.
#[cfg(target_arch = "wasm32")]
fn main() { // initialize console logging
    let _ = console_log::init_with_level(log::Level::Info);
    dioxus::launch(RudiServerBoundaries);
}

/// This `main()` function is never used, it keeps rust-analyzer happy, which 
/// expects a `main()` function in every crate and the IDE will not match to the 
/// `#[cfg]`-gated `main()` functions above.
#[cfg(all(not(feature = "server"),not(target_arch = "wasm32")))]
fn main(){}

/// `RudiServerBoundaries` provides a top-level `ErrorBoundary` and 
/// `SuspenseBoundary` for the apps interface.
#[component]
fn RudiServerBoundaries() -> Element {
    rsx! {
        ErrorBoundary {
            handle_error: |errors: ErrorContext| rsx! {
                div { class: "server-load-error",
                    p { "{errors:?}" }
                }
            },
            SuspenseBoundary {
                fallback: |_| rsx! {
                    div { class: "server-load-loading",
                        p { "Loading RuDI interface..." }
                    }
                },
                RudiServer {}
            }
        }
    }
}

/// `RudiServer` initializes the web page and layout.
#[component]
fn RudiServer() -> Element {

    // make the server config and state available as global context
    let server_config = server_config::get_server_config();
    use_context_provider(|| server_config.clone());
    use_context_provider(|| Signal::new(
        ServerState::new(&server_config.suite_config.name)
    ));
    use_context_provider(|| Signal::new(UiState::new()));

    rsx! {
        document::Link { rel: "icon", href: RUDI_LOGO_ICO }
        document::Link { rel: "stylesheet", href: DX_COMPONENTS_THEME }
        document::Link { rel: "stylesheet", href: RUDI_THEME_CSS }
        document::Link { rel: "stylesheet", href: RUDI_LAYOUT_CSS }
        document::Link { rel: "stylesheet", href: asset!("/assets/suite_stylesheet.css") }
        document::Script { src: RUDI_FRAMEWORK_JS }
        document::Script { src: asset!("/assets/suite_javascript.js") }
        RudiLayout {}
    }
}

/// `RudiLayout` creates the grid layout for the header and app step sidebar.
#[component]
fn RudiLayout() -> Element {
    let server_state = use_context::<Signal<ServerState>>();
    let ui_state = use_context::<Signal<UiState>>();
    let app_step_name = use_memo(
        move || server_state.read().get_step()
    );
    const APP_STEPS_CLASS: &str    = "app-steps-sidebar";
    const INSTRUCTIONS_CLASS: &str = "instructions-sidebar";
    const HIDDEN_CLASS: &str       = "hidden-sidebar";
    let sidebar_class = use_memo(move || {
        if ui_state.read().sidebar_open {
            if ui_state.read().showing_app_steps {
                APP_STEPS_CLASS.to_string()
            } else {
                INSTRUCTIONS_CLASS.to_string()
            }
        } else {
            HIDDEN_CLASS.to_string()
        }
    });
    rsx! {
        div { id: "page-grid-container", class: sidebar_class,
            div { id: "page-suite-label-wrapper", SuiteLabel {} }
            div { id: "page-header-content-wrapper", ServerHeaderContent {} }
            if ui_state.read().showing_app_steps {
                div { id: "app-steps-navbar-wrapper",
                    AppStepChooser {}
                    BookmarkSaver {}
                }
            } else {
                div { id: "instructions-wrapper", AppStepInstructions {} }
            }
            div { id: "main-content-wrapper", {include!(concat!(env!("OUT_DIR"), "/app_matcher.rs"))} }
        }
    }
}
