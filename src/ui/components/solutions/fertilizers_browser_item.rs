use crate::model::fertilizers::Fertilizer;
use crate::ui::components::layout::Row;
use crate::ui::components::utils::icons::ArrowRight;
use crate::ui::components::utils::Text;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersBrowserItemProps {
    fertilizer: Fertilizer,
    on_select: EventHandler<String>,
}

#[component]
pub fn FertilizersBrowserItem(props: FertilizersBrowserItemProps) -> Element {
    let fertilizer = use_signal(|| props.fertilizer);

    rsx! {
        div {
            class: "fertilizers-browser__item",
            onclick: move |_| {
                props.on_select.call(fertilizer.read().id());
            },

            Row {
                gap: "medium",
                horizontal: "space-between",
                vertical: "center",

                Text {
                    size: "x-small",
                    {fertilizer.read().name()},
                }

                ArrowRight {}
            }
        }
    }
}
