use leptos::prelude::*;

#[component]
pub fn SearchBar(
    value: ReadSignal<String>,
    on_input: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <div class="search-bar">
            <input
                type="text"
                placeholder="Search templates..."
                prop:value=value
                on:input=move |ev| {
                    on_input.set(event_target_value(&ev));
                }
            />
        </div>
    }
}
