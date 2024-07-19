use super::FertilizersBrowserTooltip;
use crate::model::fertilizers::Fertilizer;
use crate::ui::components::utils::{QuickAction, Text};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersBrowserItemProps {
    fertilizer: Signal<Fertilizer>,
    on_select: EventHandler<String>,
}

#[component]
pub fn FertilizersBrowserItem(props: FertilizersBrowserItemProps) -> Element {
    rsx! {
        QuickAction {
            action_right: rsx! {
                FertilizersBrowserTooltip {
                    fertilizer: props.fertilizer,
                },
            },
            on_click: move |_| {
                props.on_select.call(props.fertilizer.read().id());
            },

            Text {
                size: "x-small",
                {props.fertilizer.read().name()},
            }
        }
    }
}
