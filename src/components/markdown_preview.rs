use leptos::prelude::*;
use comrak::{parse_document, Arena, Options};

fn render_markdown(input: &str) -> String {
    let arena = Arena::new();
    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.render.r#unsafe = true;

    let root = parse_document(&arena, input, &options);
    let mut html = String::new();
    comrak::format_html(root, &options, &mut html).unwrap();
    html
}

#[component]
pub fn MarkdownPreview(content: Signal<String>) -> impl IntoView {
    let html = move || render_markdown(&content.get());

    view! {
        <div class="markdown-preview prose" inner_html=html />
    }
}
