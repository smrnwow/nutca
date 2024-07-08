use dioxus::prelude::*;

fn button_class(current_button: String, button_value: String) -> String {
    if current_button == button_value {
        String::from("buttons-group__button buttons-group__button_active")
    } else {
        String::from("buttons-group__button")
    }
}

#[derive(Clone, PartialEq)]
pub struct ButtonsGroupButton {
    pub label: String,
    pub value: String,
    pub badge: Element,
}

#[derive(Props, PartialEq, Clone)]
pub struct ButtonsGroupProps {
    buttons: Vec<ButtonsGroupButton>,
    value: String,
    #[props(default = String::from("small"))]
    size: String,
    on_change: EventHandler<String>,
}

#[component]
pub fn ButtonsGroup(props: ButtonsGroupProps) -> Element {
    rsx! {
        div {
            class: "buttons-group buttons-group_size-{props.size}",

            for button in props.buttons {
                button {
                    class: button_class(props.value.clone(), button.value.clone()),
                    onclick: move |_| props.on_change.call(button.value.clone()),
                    {button.label},
                    {button.badge},
                }
            }
        }
    }
}
