use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TitleProps {
    id: Option<String>,
    size: Option<String>,
    children: Option<Element>,
}

#[component]
pub fn Title(props: TitleProps) -> Element {
    let size = props.size.unwrap_or(String::from("medium"));

    let id = props.id.unwrap_or(String::new());

    let mut show_reference = use_signal(|| false);

    rsx! {
        h3 {
            id: "{id}",
            class: "title title_size-{size}",
            onmouseover: move |_| {
                show_reference.set(true);
            },
            onmouseout: move |_| {
                show_reference.set(false);
            },

            {props.children},
        }
    }
}
