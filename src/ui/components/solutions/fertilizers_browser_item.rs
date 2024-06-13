use crate::model::fertilizers::Fertilizer;
use crate::ui::components::utils::{Badge, Checkbox, TableCell, TableRow};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersBrowserItemProps {
    fertilizer: Fertilizer,
    selected: bool,
    on_select: EventHandler<Fertilizer>,
    on_remove: EventHandler<String>,
}

#[component]
pub fn FertilizersBrowserItem(props: FertilizersBrowserItemProps) -> Element {
    let fertilizer = use_signal(|| props.fertilizer);

    rsx! {
        TableRow {
            TableCell {
                div {
                    class: "fertilizers-browser__selector",

                    Checkbox {
                        checked: props.selected,
                        on_change: move |event: Event<FormData>| {
                            let selected: bool = event.value().parse().unwrap();

                            if selected {
                                props.on_select.call(fertilizer.read().clone());
                            } else {
                                props.on_remove.call(fertilizer.read().id());
                            }
                        }
                    }
                }
            }

            TableCell {
                p {
                    class: "fertilizers-browser__name",
                    "{fertilizer.read().name()}",
                }
            }

            TableCell {
                div {
                    class: "fertilizers-browser__nutrients",

                    for nutrient in fertilizer.read().nutrients() {
                        Badge {
                            text: nutrient.symbol(),
                        }
                    }
                }
            }
        }
    }
}
