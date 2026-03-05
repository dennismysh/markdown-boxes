use leptos::prelude::*;
use crate::models::filter::Filter;
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

    let is_required = placeholder.filters.iter().any(|f| matches!(f, Filter::Required));
    let max_length = placeholder.filters.iter().find_map(|f| match f {
        Filter::MaxLength(n) => Some(*n),
        _ => None,
    });

    let label_suffix = if is_required { " *" } else { "" };
    let full_label = format!("{label_text}{label_suffix}");

    let input_view = match placeholder.kind {
        PlaceholderType::Text => {
            let ph = format!("Enter {}...", input_label);
            let k = key.clone();
            let ml = max_length;
            view! {
                <input
                    type="text"
                    id=k
                    prop:value=value
                    on:input=move |ev| on_change.set(event_target_value(&ev))
                    placeholder=ph
                    maxlength=ml.map(|n| n.to_string()).unwrap_or_default()
                />
                {ml.map(|max| {
                    view! {
                        <span class="char-counter">
                            {move || format!("{}/{}", value.get().len(), max)}
                        </span>
                    }
                })}
            }.into_any()
        },
        PlaceholderType::Multiline => {
            let ph = format!("Enter {}...", input_label);
            let k = key.clone();
            let ml = max_length;
            view! {
                <textarea
                    id=k
                    prop:value=value
                    on:input=move |ev| on_change.set(event_target_value(&ev))
                    placeholder=ph
                    rows=4
                    maxlength=ml.map(|n| n.to_string()).unwrap_or_default()
                />
                {ml.map(|max| {
                    view! {
                        <span class="char-counter">
                            {move || format!("{}/{}", value.get().len(), max)}
                        </span>
                    }
                })}
            }.into_any()
        },
        PlaceholderType::Select => {
            let filter_options = placeholder.filters.iter().find_map(|f| match f {
                Filter::Options(opts) => Some(opts.clone()),
                _ => None,
            });
            let options = filter_options.or(placeholder.options).unwrap_or_default();
            let mobile_options = options.clone();
            let k = key.clone();
            let k2 = k.clone();
            view! {
                <select
                    id=k
                    class="select-desktop"
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
                <div class="select-mobile" id=k2>
                    {mobile_options.into_iter().map(|opt| {
                        let val = opt.clone();
                        let click_val = val.clone();
                        let display = opt.clone();
                        view! {
                            <button
                                type="button"
                                class=move || {
                                    if value.get() == val { "select-mobile-option selected" } else { "select-mobile-option" }
                                }
                                on:click=move |_| on_change.set(click_val.clone())
                            >
                                {display}
                            </button>
                        }
                    }).collect_view()}
                </div>
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

    let field_class = if is_required { "form-field required" } else { "form-field" };

    view! {
        <div class=field_class>
            <label for=key>{full_label}</label>
            {input_view}
        </div>
    }
}
