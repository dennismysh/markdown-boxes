use std::collections::HashMap;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use wasm_bindgen::JsCast;
use crate::components::form_field::FormField;
use crate::components::markdown_preview::MarkdownPreview;
use crate::models::filter::Filter;
use crate::models::placeholder::Placeholder;
use crate::engine::evaluate_blocks;
use crate::store;
use crate::substitute::substitute;

#[component]
pub fn TemplateView() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.read().get("slug").unwrap_or_default();

    let all_templates = store::load_templates();

    let template = move || store::get_template(&all_templates, &slug());

    let back_href = format!("{}/", crate::BASE_PATH);

    view! {
        {move || {
            let back = back_href.clone();
            if let Some(tmpl) = template() {
                let title = tmpl.title.clone();
                let body = tmpl.body.clone();
                let placeholders = tmpl.placeholders.clone();
                let tmpl_sections = tmpl.sections.clone();
                let preview_src = tmpl.preview.map(|p| format!("{}/previews/{p}", crate::BASE_PATH));

                let field_signals: Vec<(Placeholder, ReadSignal<String>, WriteSignal<String>)> =
                    placeholders.into_iter().map(|p| {
                        let default_val = p.filters.iter().find_map(|f| match f {
                            Filter::Default(v) => Some(v.clone()),
                            _ => None,
                        }).unwrap_or_default();
                        let (read, write) = signal(default_val);
                        (p, read, write)
                    }).collect();

                let field_signals_for_preview = field_signals.clone();

                let preview_content = Signal::derive(move || {
                    let mut values = HashMap::new();
                    for (p, read, _) in &field_signals_for_preview {
                        let val = read.get();
                        if !val.is_empty() {
                            values.insert(p.key.clone(), val);
                        }
                    }
                    let after_blocks = evaluate_blocks(&body, &values);
                    substitute(&after_blocks, &values)
                });

                view! {
                    <div class="template-view">
                        <div class="template-header">
                            <a href=back class="back-link">"← Back to gallery"</a>
                            <h2>{title}</h2>
                        </div>
                        <div class="template-layout">
                            <div class="template-form">
                                <h3>"Customize"</h3>
                                {if !tmpl_sections.is_empty() {
                                    Some(view! {
                                        <div class="section-badges">
                                            {tmpl_sections.iter().map(|s| {
                                                let name = s.name.clone();
                                                let stype = format!("{:?}", s.section_type).to_lowercase();
                                                view! {
                                                    <span class="section-badge">
                                                        <span class="section-badge-type">{stype}</span>
                                                        {name}
                                                    </span>
                                                }
                                            }).collect_view()}
                                        </div>
                                    })
                                } else {
                                    None
                                }}
                                {field_signals.into_iter().map(|(p, read, write)| {
                                    view! { <FormField placeholder=p value=read on_change=write /> }
                                }).collect_view()}
                                <div class="export-buttons">
                                    <ExportButtons content=preview_content />
                                </div>
                            </div>
                            <div class="template-preview-pane">
                                <h3>"Preview"</h3>
                                {preview_src.map(|src| view! {
                                    <div class="hero-diagram">
                                        <img src=src alt="Template diagram" />
                                    </div>
                                })}
                                <MarkdownPreview content=preview_content />
                            </div>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! {
                    <div class="not-found">
                        <h2>"Template not found"</h2>
                        <a href=back>"← Back to gallery"</a>
                    </div>
                }.into_any()
            }
        }}
    }
}

#[component]
fn ExportButtons(content: Signal<String>) -> impl IntoView {
    let (copied, set_copied) = signal(false);

    let copy_to_clipboard = move |_| {
        let text = content.get();
        let window = web_sys::window().unwrap();
        let navigator = window.navigator();
        let clipboard = navigator.clipboard();
        let _ = clipboard.write_text(&text);
        set_copied.set(true);
        leptos::prelude::set_timeout(move || set_copied.set(false), std::time::Duration::from_secs(2));
    };

    let download = move |_| {
        let text = content.get();
        let blob_parts = js_sys::Array::new();
        blob_parts.push(&wasm_bindgen::JsValue::from_str(&text));
        let opts = web_sys::BlobPropertyBag::new();
        opts.set_type("text/markdown");
        let blob = web_sys::Blob::new_with_str_sequence_and_options(&blob_parts, &opts).unwrap();
        let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

        let document = web_sys::window().unwrap().document().unwrap();
        let a: web_sys::HtmlAnchorElement = document.create_element("a").unwrap().unchecked_into();
        a.set_href(&url);
        a.set_download("template.md");
        a.click();
        web_sys::Url::revoke_object_url(&url).unwrap();
    };

    view! {
        <button class="btn btn-primary" on:click=copy_to_clipboard>
            {move || if copied.get() { "Copied!" } else { "Copy to Clipboard" }}
        </button>
        <button class="btn btn-secondary" on:click=download>
            "Download .md"
        </button>
    }
}
