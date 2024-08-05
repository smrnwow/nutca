use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct BannerProps {
    size: Option<String>,
    state: Option<String>,
    more_link: Option<String>,
    text: String,
}

#[component]
pub fn Banner(props: BannerProps) -> Element {
    let size = props.size.unwrap_or(String::from("medium"));

    let state = props.state.unwrap_or(String::from("default"));

    rsx! {
        div {
            class: "banner banner_size-{size} banner_state-{state}",

            div {
                class: "banner__body",

                div {
                    class: "banner__icon",
                }

                p {
                    class: "banner__text",
                    {props.text},
                }
            }

            div {
                class: "banner__links",

                if let Some(more_link) = props.more_link {
                    a {
                        class: "banner__more-link",
                        href: more_link,
                        "Подробнее",
                    }
                }
            }
        }
    }
}
