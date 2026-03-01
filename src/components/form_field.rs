use leptos::prelude::*;
use crate::models::placeholder::{Placeholder, PlaceholderType};

#[component]
pub fn FormField(
    placeholder: Placeholder,
    value: ReadSignal<String>,
    on_change: WriteSignal<String>,
) -> impl IntoView {
    let label_text = placeholder.label.clone();
    let key = placeholder.key.clone();
    let input_label = placeholder.label.clone();

    let input_view = match placeholder.kind {
        PlaceholderType::Text => {
            let ph = format!("Enter {}...", input_label);
            let k = key.clone();
            view! {
                <input
                    type="text"
                    id=k
                    prop:value=value
                    on:input=move |ev| on_change.set(event_target_value(&ev))
                    placeholder=ph
                />
            }.into_any()
        },
        PlaceholderType::Multiline => {
            let ph = format!("Enter {}...", input_label);
            let k = key.clone();
            view! {
                <textarea
                    id=k
                    prop:value=value
                    on:input=move |ev| on_change.set(event_target_value(&ev))
                    placeholder=ph
                    rows=4
                />
            }.into_any()
        },
        PlaceholderType::Select => {
            let options = placeholder.options.unwrap_or_default();
            let k = key.clone();
            view! {
                <select
                    id=k
                    on:change=move |ev| on_change.set(event_target_value(&ev))
                >
                    <option value="" disabled selected>"Select..."</option>
                    {options.into_iter().map(|opt| {
                        let val = opt.clone();
                        view! {
                            <option value=val>{opt}</option>
                        }
                    }).collect_view()}
                </select>
            }.into_any()
        },
        PlaceholderType::Boolean => {
            view! {
                <label class="toggle-label">
                    <input
                        type="checkbox"
                        on:change=move |ev| {
                            let checked = event_target_checked(&ev);
                            on_change.set(if checked { "Yes".to_string() } else { "No".to_string() });
                        }
                    />
                    <span>{move || value.get()}</span>
                </label>
            }.into_any()
        },
    };

    view! {
        <div class="form-field">
            <label for=key>{label_text}</label>
            {input_view}
        </div>
    }
}
