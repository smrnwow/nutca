use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SwitchToggleProps {
    options: Vec<(String, String)>,
    value: String,
}

#[component]
pub fn SwitchToggle(props: SwitchToggleProps) -> Element {
    rsx! {
        div {
            class: "fertilizers-source__tabs",

            for (value, text) in props.options {
                button {
                    class: "{tab_active_class(fertilizer.is_label_based())}",
                    onclick: move |_| props.on_label_select.call(()),
                    "С этикетки"
                }
            }


            button {
                class: "{tab_active_class(fertilizer.is_formula_based())}",
                onclick: move |_| props.on_formula_select.call(()),
                "По формуле"
            }
        }
    }
}
