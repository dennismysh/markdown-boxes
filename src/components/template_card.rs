use leptos::prelude::*;
use crate::models::template::Template;

#[component]
pub fn TemplateCard(template: Template) -> impl IntoView {
    let href = format!("/template/{}", template.slug);
    let category_label = template.category.label().to_string();
    let title = template.title.clone();
    let description = template.description.clone();
    let tags = template.tags.clone();
    let has_preview = template.preview.is_some();
    let preview_src = template.preview.map(|p| format!("/previews/{p}"));

    view! {
        <a href=href class="template-card">
            <div class="card-preview">
                {if has_preview {
                    view! {
                        <img src=preview_src.unwrap() alt="" />
                    }.into_any()
                } else {
                    view! {
                        <div class="card-preview-placeholder">
                            <span>{category_label.clone()}</span>
                        </div>
                    }.into_any()
                }}
            </div>
            <div class="card-body">
                <span class="card-category">{category_label}</span>
                <h3 class="card-title">{title}</h3>
                <p class="card-description">{description}</p>
                <div class="card-tags">
                    {tags.into_iter().map(|tag| view! {
                        <span class="tag">{tag}</span>
                    }).collect_view()}
                </div>
            </div>
        </a>
    }
}
