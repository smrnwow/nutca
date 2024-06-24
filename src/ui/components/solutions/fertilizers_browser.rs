use super::FertilizersBrowserItem;
use crate::model::fertilizers::Fertilizer;
use crate::ui::components::layout::Column;
use crate::ui::components::utils::{Search, Title};
use dioxus::prelude::*;

fn round(value: f64) -> String {
    format!("{:.3}", value)
}

fn units(liquid: bool) -> String {
    match liquid {
        true => String::from("мл"),
        false => String::from("г"),
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersBrowserProps {
    fertilizers: Memo<Vec<Fertilizer>>,
    on_select: EventHandler<String>,
    on_search: EventHandler<String>,
}

#[component]
pub fn FertilizersBrowser(props: FertilizersBrowserProps) -> Element {
    rsx! {
        Column {
            Title {
                size: "small",
                text: "Выбор удобрений",
            }

            Search {
                placeholder: "найти удобрение",
                on_change: props.on_search,
            }

            div {
                class: "fertilizers-browser__table",

                for fertilizer in props.fertilizers.read().clone() {
                    FertilizersBrowserItem {
                        key: "{fertilizer.id()}",
                        fertilizer,
                        on_select: props.on_select,
                    }
                }
            }
        }
    }
}
