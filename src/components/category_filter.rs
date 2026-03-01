use leptos::prelude::*;
use crate::models::template::Category;

#[component]
pub fn CategoryFilter(
    selected: ReadSignal<Option<Category>>,
    on_select: WriteSignal<Option<Category>>,
) -> impl IntoView {
    let categories = vec![
        Category::ImplementationPlan,
        Category::DesignPrompt,
        Category::UiComponent,
        Category::FullStackFlow,
        Category::BackendPattern,
    ];

    view! {
        <div class="category-filter">
            <button
                class=move || if selected.get().is_none() { "filter-btn active" } else { "filter-btn" }
                on:click=move |_| on_select.set(None)
            >
                "All"
            </button>
            {categories.into_iter().map(|cat| {
                let cat_for_check = cat.clone();
                let cat_for_click = cat.clone();
                let label = cat.label();
                view! {
                    <button
                        class=move || {
                            if selected.get().as_ref() == Some(&cat_for_check) { "filter-btn active" } else { "filter-btn" }
                        }
                        on:click=move |_| on_select.set(Some(cat_for_click.clone()))
                    >
                        {label}
                    </button>
                }
            }).collect_view()}
        </div>
    }
}
