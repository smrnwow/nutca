use crate::ui::components::utils::{Button, Reference};
use dioxus::prelude::*;

fn state(props: &QuickActionProps) -> String {
    if let Some(error) = props.error {
        if error {
            return String::from("error");
        }
    }

    if let Some(warn) = props.warn {
        if warn {
            return String::from("warn");
        }
    }

    if let Some(success) = props.success {
        if success {
            return String::from("success");
        }
    }

    String::from("default")
}

#[derive(Props, PartialEq, Clone)]
pub struct QuickActionProps {
    size: Option<String>,
    children: Option<Element>,
    error: Option<bool>,
    warn: Option<bool>,
    success: Option<bool>,
    action_left: Option<Element>,
    action_right: Option<Element>,
    reference: Option<Element>,
    on_click: Option<EventHandler<()>>,
    on_hover_in: Option<EventHandler<()>>,
    on_hover_out: Option<EventHandler<()>>,
}

#[component]
pub fn QuickAction(props: QuickActionProps) -> Element {
    let state = state(&props);

    let size = props.size.unwrap_or(String::from("small"));

    let has_reference = use_signal(|| props.reference.is_some());

    let mut action_hidden = use_signal(|| false && *has_reference.read());

    let mut show_reference = use_signal(|| false);

    rsx! {
        div {
            class: "quick-action quick-action_state-{state} quick-action_size-{size}",
            onclick: move |_| {
                if let Some(on_click) = props.on_click {
                    on_click.call(());
                }
            },
            onmouseover: move |_| {
                action_hidden.set(true && *has_reference.read());

                show_reference.set(true);

                if let Some(on_hover_in) = props.on_hover_in {
                    on_hover_in.call(());
                }
            },
            onmouseout: move |_| {
                show_reference.set(false);

                action_hidden.set(false && *has_reference.read());

                if let Some(on_hover_out) = props.on_hover_out {
                    on_hover_out.call(());
                }
            },

            if let Some(action_left) = props.action_left {
                div {
                    class: "quick-action__left-action quick-action__left-action_has-reference-{*has_reference.read()}",

                    Button {
                        style: "compact",
                        {action_left},
                    }

                    if let Some(reference) = props.reference.clone() {
                        Reference {
                            display: Signal::new(false),
                            style: "button",
                            tooltip: reference,
                            tooltip_position: "top-left",
                        }
                    }
                }
            }

            div {
                class: "quick-action__text quick-action",
                {props.children},
            }

            if let Some(action_right) = props.action_right {
                div {
                    class: "quick-action__right-action quick-action__right-action_has-reference-{*has_reference.read()}",

                    Button {
                        style: "compact",
                        {action_right},
                    }

                    if let Some(reference) = props.reference.clone() {
                        Reference {
                            display: Signal::new(false),
                            style: "button",
                            tooltip: reference,
                            tooltip_position: "top-right",
                        }
                    }
                }
            }
        }
    }
}
