pub mod components;
pub mod engine;
pub mod models;
pub mod pages;
pub mod store;
pub mod substitute;

use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use pages::gallery::Gallery;
use pages::template_view::TemplateView;

#[cfg(debug_assertions)]
pub const BASE_PATH: &str = "";
#[cfg(not(debug_assertions))]
pub const BASE_PATH: &str = "/markdown-boxes";

#[component]
pub fn App() -> impl IntoView {
    let home_href = format!("{}/", BASE_PATH);

    view! {
        <Router base=BASE_PATH>
            <div class="app">
                <header>
                    <a href=home_href class="logo">"Markdown Boxes"</a>
                </header>
                <main>
                    <Routes fallback=|| view! { <p class="not-found">"Not found"</p> }>
                        <Route path=path!("/") view=Gallery/>
                        <Route path=path!("/template/:slug") view=TemplateView/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
