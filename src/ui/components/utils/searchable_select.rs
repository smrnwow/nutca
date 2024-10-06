use super::icons::{ArrowDown, ArrowUp, Check, Close, SearchIcon};
use dioxus::prelude::*;
use std::rc::Rc;

fn select_list_class(opened: bool) -> String {
    /*
    if opened {
        String::from("select__list select__list_opened")
    } else {
        String::from("select__list")
    }
    */

    String::from("select__list")
}

#[derive(Props, PartialEq, Clone)]
pub struct SearchableSelectProps {
    value: Memo<(String, String)>,
    options: Vec<(String, String)>,
    placeholder: Option<String>,
    search_query: Option<String>,
    size: Option<String>,
    on_search: Option<EventHandler<String>>,
    on_change: EventHandler<String>,
}

#[component]
pub fn SearchableSelect(props: SearchableSelectProps) -> Element {
    let size = props.size.unwrap_or(String::from("small"));

    let mut is_opened = use_signal(|| false);

    let show_value = use_memo(move || props.value.read().0.len() > 0 && !*is_opened.read());

    let mut search_input: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    let search_query = use_signal(|| props.search_query.unwrap_or(String::new()));

    let show_search = use_memo(move || props.on_search.is_some() && !*show_value.read());

    let placeholder = use_signal(|| props.placeholder.unwrap_or(String::from("выберите")));

    let show_cancel = use_memo(move || props.on_search.is_some() && props.value.read().0.len() > 0);

    let show_placeholder = use_memo(move || !*show_value.read() && !*show_search.read());

    rsx! {
        div {
            class: "select select_size-{size}",

            label {
                class: "select__header",

                if props.value.read().0.len() > 0 && !*is_opened.read() {
                    button {
                        class: "select__value",
                        onclick: move |_| async move {
                            is_opened.set(true);
                        },
                        onfocusin: move |_| {
                            is_opened.set(true);
                        },
                        {props.value.read().1.clone()},
                    }
                }

                if props.on_search.is_some() && (*is_opened.read() || props.value.read().0.len() == 0) {
                    label {
                        class: "select__search",
                        onclick: move |_| {
                            is_opened.set(true);
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
                            onmounted: move |event| async move {
                                if *is_opened.read() {
                                    let _ = event.data().set_focus(true).await;
                                }
                            },
                            oninput: move |search_query| {
                                if let Some(on_search) = props.on_search {
                                    on_search.call(search_query.value());
                                }
                            },
                        }
                    }
                }

                /*
                if *show_placeholder.read() {
                    button {
                        class: "select__placeholder",
                        onclick: move |_| {
                            is_opened.set(true);
                        },
                        onfocusin: move |_| {
                            is_opened.set(true);
                        },
                        {placeholder},
                    }
                }
                */

                SelectControls {
                    show_cancel,
                    arrow_direction_up: is_opened,
                    on_cancel: move |_| {
                        props.on_change.call(String::new());

                        is_opened.set(true);
                    },
                }
            }

            SelectList {
                is_opened,
                options: props.options,
                value: props.value.read().1.clone(),
                on_select: move |new_value| {
                    if new_value != *props.value.read().1 {
                        props.on_change.call(new_value);
                    }

                    is_opened.set(false);
                },
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
    rsx! {}
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
