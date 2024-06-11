use crate::model::fertilizers::Fertilizer;
use crate::ui::components::utils::{Badge, TableCell, TableRow};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizerListingItemProps {
    fertilizer: Fertilizer,
    on_select: EventHandler<String>,
}

#[component]
pub fn FertilizerListingItem(props: FertilizerListingItemProps) -> Element {
    let fertilizer_id = props.fertilizer.id();

    rsx! {
        TableRow {
            on_click: move |_| props.on_select.call(fertilizer_id.clone()),

            TableCell {
                p {
                    class: "fertilizer-listing__name",
                    "{props.fertilizer.name()}",
                }

                p {
                    class: "fertilizer-listing__vendor",
                    "Производитель: {props.fertilizer.vendor()}",
                }
            }

            TableCell {
                div {
                    class: "fertilizer-listing__nutrients",

                    for nutrient in props.fertilizer.nutrients() {
                        Badge {
                            text: nutrient.symbol(),
                        }
                    }
                }
            }
        }
    }
}
