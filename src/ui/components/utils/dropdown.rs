use super::icons::{ArrowDown, ArrowUp, Check, Close, SearchIcon};
use dioxus::prelude::*;

fn dropdown_list_class(opened: bool) -> String {
    if opened {
        String::from("dropdown__list dropdown__list_opened")
    } else {
        String::from("dropdown__list")
    }
}

fn dropdown_item_class(selected: bool) -> String {
    if selected {
        String::from("dropdown__item dropdown__item_selected")
    } else {
        String::from("dropdown__item")
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct DropdownProps {
    placeholder: Option<String>,
    value: (String, String),
    options: Vec<(String, String)>,
    on_search: Option<EventHandler<String>>,
    on_select: EventHandler<String>,
    on_cancel: EventHandler<()>,
}

#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    let mut search_query = use_signal(|| String::new());

    let mut opened = use_signal(|| false);

    let placeholder = props.placeholder.unwrap_or(String::new());

    let (value, text) = props.value.clone();

    let render_value = value != String::from("") && !*opened.read();

    let render_search = !render_value && props.on_search.is_some();

    let render_placeholder = !render_value && !render_search;

    rsx! {
        div {
            class: "dropdown",

            div {
                class: "dropdown__header",

                if render_value {
                    button {
                        class: "dropdown__value",
                        onclick: move |_| {
                            *opened.write() = true;
                        },

                        "{text}",
                    }
                }

                if render_search {
                    label {
                        class: "dropdown__search",
                        onclick: move |_| {
                            *opened.write() = true;
                        },
                        /*
                        onfocusout: move |_| {
                            *opened.write() = false;
                        },
                        */

                        div {
                            class: "dropdown__icon",

                            SearchIcon {}
                        }

                        input {
                            class: "dropdown__input",
                            r#type: "text",
                            placeholder: "{placeholder}",
                            value: "{search_query}",
                            oninput: move |event| {
                                *search_query.write() = event.value();

                                props.on_search.unwrap().call(search_query.read().clone())
                            },
                        }
                    }
                }

                if render_placeholder {
                        button {
                            class: "dropdown__placeholder",
                            onclick: move |_| {
                                *opened.write() = true;
                            },

                            "{placeholder}"
                        }
                    }
                }

                div {
                    class: "dropdown__controls",

                    if value != String::new() {
                        button {
                            class: "dropdown__cancel",
                            onclick: move |_| props.on_cancel.call(()),

                            Close {}
                        }
                    }

                    div {
                        class: "dropdown__arrow",

                        if *opened.read() {
                            ArrowUp {}
                        } else {
                            ArrowDown {}
                        }
                    }
                }

                ul {
                    class: dropdown_list_class(*opened.read()),

                    for (option_value, option_text) in props.options.clone() {
                        li {
                            class: dropdown_item_class(option_value == value),

                            button {
                                class: "dropdown__button",
                                onclick: move |_| {
                                    *opened.write() = false;

                                    props.on_select.call(option_value.clone());
                                },

                                "{option_text}",
                            }

                            if value == option_value {
                                div {
                                    class: "dropdown__check",

                                    Check {}
                                }
                            }
                        }
                    }
                }
            }
    }
}
