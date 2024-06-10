use crate::model::fertilizers::Fertilizer;
use crate::ui::components::utils::{Badge, Checkbox, TableCell, TableRow};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersBrowserItemProps {
    fertilizer: Signal<Fertilizer>,
    selected: bool,
    on_select: EventHandler<(bool, Fertilizer)>,
}

#[component]
pub fn FertilizersBrowserItem(props: FertilizersBrowserItemProps) -> Element {
    rsx! {
        TableRow {
            key: "{props.fertilizer.read().id()}",

            TableCell {
                div {
                    class: "fertilizers-browser__selector",

                    Checkbox {
                        checked: props.selected,
                        on_change: move |event: Event<FormData>| {
                            props.on_select.call((event.value().parse().unwrap(), props.fertilizer.read().clone()));
                        }
                    }
                }
            }

            TableCell {
                p {
                    class: "fertilizers-browser__name",
                    "{props.fertilizer.read().name()}",
                }
            }

            TableCell {
                div {
                    class: "fertilizers-browser__nutrients",

                    for nutrient in props.fertilizer.read().nutrients() {
                        Badge {
                            text: nutrient.symbol(),
                        }
                    }
                }
            }
        }
    }
}
