pub mod models;
pub mod store;

use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="app">
                <header>
                    <h1>"Markdown Boxes"</h1>
                </header>
                <main>
                    <Routes fallback=|| view! { <p>"Not found"</p> }>
                        <Route path=path!("/") view=|| view! { <p>"Gallery coming soon"</p> }/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
