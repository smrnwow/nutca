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

    rsx! {
        div {
            class: "quick-action quick-action_state-{state} quick-action_size-{size}",
            onclick: move |_| {
                if let Some(on_click) = props.on_click {
                    on_click.call(());
                }
            },

            if let Some(action_left) = props.action_left {
                div {
                    class: "quick-action__left-action",
                    {action_left},
                }
            }

            div {
                class: "quick-action__text",
                {props.children},
            }

            if let Some(action_right) = props.action_right {
                div {
                    class: "quick-action__right-action",
                    {action_right},
                }
            }
        }
    }
}
