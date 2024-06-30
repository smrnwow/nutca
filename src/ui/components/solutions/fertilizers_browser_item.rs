use crate::model::fertilizers::Fertilizer;
use crate::ui::components::solutions::FertilizersBrowserTooltip;
use crate::ui::components::utils::icons::ArrowRight;
use crate::ui::components::utils::{QuickAction, Text};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersBrowserItemProps {
    fertilizer: Fertilizer,
    on_select: EventHandler<String>,
}

#[component]
pub fn FertilizersBrowserItem(props: FertilizersBrowserItemProps) -> Element {
    let fertilizer = use_signal(|| props.fertilizer);

    let mut display_reference = use_signal(|| false);

    rsx! {
        QuickAction {
            action_right: rsx! {
                ArrowRight {},
            },
            reference: rsx! {
                FertilizersBrowserTooltip {
                    fertilizer,
                },
            },
            on_click: move |_| {
                props.on_select.call(fertilizer.read().id());
            },
            on_hover_in: move |_| {
                display_reference.set(true);
            },
            on_hover_out: move |_| {
                display_reference.set(false);
            },

            Text {
                size: "x-small",
                {fertilizer.read().name()},
            }
        }
    }
}
