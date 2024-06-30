use super::icons::{ArrowDown, ArrowUp, Check, Close, SearchIcon};
use dioxus::prelude::*;

fn select_list_class(opened: bool) -> String {
    if opened {
        String::from("select__list select__list_opened")
    } else {
        String::from("select__list")
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct SelectProps {
    value: Memo<(String, String)>,
    placeholder: Option<String>,
    search_query: Option<String>,
    label: Option<String>,
    size: Option<String>,
    options: Vec<(String, String)>,
    on_search: Option<EventHandler<String>>,
    on_change: EventHandler<String>,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
    let size = props.size.unwrap_or(String::from("small"));

    let mut is_opened = use_signal(|| false);

    let search_query = use_signal(|| props.search_query.unwrap_or(String::new()));

    let placeholder = use_signal(|| props.placeholder.unwrap_or(String::from("выберите")));

    let value = use_memo(move || props.value.read().0.clone());

    let text = use_memo(move || props.value.read().1.clone());

    let show_value = use_memo(move || value.read().len() > 0 && !*is_opened.read());

    let show_cancel = use_memo(move || props.on_search.is_some() && value.read().len() > 0);

    let show_search = use_memo(move || props.on_search.is_some() && *is_opened.read());

    let show_placeholder = use_memo(move || !*show_value.read() && !*show_search.read());

    rsx! {
        div {
            class: "select select_size-{size}",
            onfocusout: move |_| {
                // *is_opened.write() = false;
            },

            if let Some(label) = props.label {
                span {
                    class: "select__label",
                    {label},
                }
            }

            SelectHead {
                arrow_direction_up: is_opened,
                show_value,
                show_cancel,
                show_search,
                show_placeholder,
                search_query,
                text,
                placeholder,
                on_change: props.on_change,
                on_open: move |_| {
                    is_opened.set(true);
                },
                on_cancel: move |_| {
                    props.on_change.call(String::new());

                    is_opened.set(true);
                },
                on_search: move |search_query| {
                    if let Some(on_search) = props.on_search {
                        on_search.call(search_query);
                    }
                },
            }

            SelectList {
                is_opened,
                options: props.options,
                value,
                on_select: move |new_value| {
                    if new_value != *value.read() {
                        props.on_change.call(new_value);
                    }

                    is_opened.set(false);
                },
            }
        }
    }
}

#[component]
fn SelectHead(
    arrow_direction_up: Signal<bool>,
    show_value: Memo<bool>,
    show_cancel: Memo<bool>,
    show_search: Memo<bool>,
    show_placeholder: Memo<bool>,
    text: Memo<String>,
    search_query: Signal<String>,
    placeholder: Signal<String>,
    on_open: EventHandler<()>,
    on_change: EventHandler<String>,
    on_cancel: EventHandler<()>,
    on_search: EventHandler<String>,
) -> Element {
    rsx! {
        label {
            class: "select__header",

            if *show_value.read() {
                button {
                    class: "select__value",
                    onclick: move |_| on_open.call(()),
                    onfocusin: move |_| on_open.call(()),
                    {text},
                }
            }

            if *show_search.read() {
                SelectSearch {
                    search_query,
                    placeholder,
                    on_open,
                    on_search,
                }
            }

            if *show_placeholder.read() {
                button {
                    class: "select__placeholder",
                    onclick: move |_| on_open.call(()),
                    onfocusin: move |_| on_open.call(()),
                    {placeholder},
                }
            }

            SelectControls {
                show_cancel,
                arrow_direction_up,
                on_cancel,
            }
        }
    }
}

#[component]
fn SelectSearch(
    search_query: String,
    placeholder: String,
    on_open: EventHandler<()>,
    on_search: EventHandler<String>,
) -> Element {
    rsx! {
        label {
            class: "select__search",
            onclick: move |_| {
                on_open.call(());
            },

            div {
                class: "select__icon",
                SearchIcon {},
            }

            input {
                class: "select__input",
                r#type: "text",
                placeholder: placeholder,
                value: search_query,
                oninput: move |event| {
                    on_search.call(event.value());
                },
                onmounted: move |event| {
                    async move {
                        event.as_ref().set_focus(true).await.unwrap();
                    }
                },
            }
        }
    }
}

#[component]
fn SelectControls(
    show_cancel: Memo<bool>,
    arrow_direction_up: Signal<bool>,
    on_cancel: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            class: "select__controls",

            if *show_cancel.read() {
                button {
                    class: "select__cancel",
                    onclick: move |_| on_cancel.call(()),
                    Close {},
                }
            }

            div {
                class: "select__arrow",

                if *arrow_direction_up.read() {
                    ArrowUp {}
                } else {
                    ArrowDown {}
                }
            }
        }
    }
}

#[component]
fn SelectList(
    is_opened: Signal<bool>,
    options: Vec<(String, String)>,
    value: String,
    on_select: EventHandler<String>,
) -> Element {
    rsx! {
        ul {
            class: "select__list",
            class: select_list_class(*is_opened.read()),

            for (option_value, option_text) in options {
                SelectListItem {
                    text: option_text,
                    is_selected: option_value == value,
                    on_select: move |_| {
                        on_select.call(option_value.clone());
                    }
                }
            }
        }
    }
}

#[component]
fn SelectListItem(text: String, is_selected: bool, on_select: EventHandler<()>) -> Element {
    let class = if is_selected {
        String::from("select__item select__item_selected")
    } else {
        String::from("select__item")
    };

    rsx! {
        li {
            class,

            button {
                class: "select__button",
                onclick: move |_| on_select.call(()),
                {text},
            }

            if is_selected {
                div {
                    class: "select__check",
                    Check {},
                }
            }
        }
    }
}
