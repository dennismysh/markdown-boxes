use leptos::prelude::*;
use crate::components::category_filter::CategoryFilter;
use crate::components::search_bar::SearchBar;
use crate::components::template_card::TemplateCard;
use crate::models::template::Category;
use crate::store;

#[component]
pub fn Gallery() -> impl IntoView {
    let all_templates = store::load_templates();
    let (search_query, set_search_query) = signal(String::new());
    let (selected_category, set_selected_category) = signal(Option::<Category>::None);

    let filtered = Memo::new(move |_| {
        let mut templates = store::search_templates(&all_templates, &search_query.get());
        if let Some(ref cat) = selected_category.get() {
            templates = store::filter_by_category(&templates, cat);
        }
        templates
    });

    view! {
        <div class="gallery">
            <div class="gallery-header">
                <h2>"Templates"</h2>
                <p class="gallery-subtitle">"Browse proven templates for AI-driven workflows"</p>
            </div>
            <SearchBar value=search_query on_input=set_search_query />
            <CategoryFilter selected=selected_category on_select=set_selected_category />
            <div class="gallery-grid">
                {move || filtered.get().into_iter().map(|t| view! {
                    <TemplateCard template=t />
                }).collect_view()}
            </div>
            {move || {
                if filtered.get().is_empty() {
                    Some(view! {
                        <p class="empty-state">"No templates match your search."</p>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}
