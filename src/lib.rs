#![feature(async_closure)]

use cfg_if::cfg_if;
pub mod app;
pub mod auth;
pub mod components;
pub mod pages;
pub mod fileserv;
pub mod utils;
pub mod state;
pub mod server;

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::app::*;

    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(move || {
            view! { <App/> }
        });
    }
}}
