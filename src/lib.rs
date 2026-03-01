pub mod components;
pub mod models;
pub mod pages;
pub mod store;
pub mod substitute;

use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use pages::gallery::Gallery;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="app">
                <header>
                    <a href="/" class="logo">"Markdown Boxes"</a>
                </header>
                <main>
                    <Routes fallback=|| view! { <p class="not-found">"Not found"</p> }>
                        <Route path=path!("/") view=Gallery/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
