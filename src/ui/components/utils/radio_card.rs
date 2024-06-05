use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct RadioCardProps {
    name: String,
    title: String,
    subtitle: String,
    value: String,
    checked: bool,
    on_change: EventHandler<String>,
}

#[component]
pub fn RadioCard(props: RadioCardProps) -> Element {
    rsx! {
        label {
            class: "radio-card",

            div {
                class: "radio-card__text",

                span {
                    class: "radio-card__title",
                    "{props.title}",
                }

                span {
                    class: "radio-card__subtitle",
                    "{props.subtitle}",
                }
            }

            div {
                class: "radio-card__control",

                input {
                    class: "radio-card__input",
                    r#type: "radio",
                    name: "{props.name}",
                    value: "{props.value}",
                    checked: "{props.checked}",
                    onchange: move |_| props.on_change.call(String::from("")),
                }

                span {
                    class: "radio-card__alias",
                }
            }
        }
    }
}
