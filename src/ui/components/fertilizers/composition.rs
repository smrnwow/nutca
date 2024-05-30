use super::{FertilizersFormula, FertilizersLabel};
use crate::model::fertilizers::{Component, Composition, Units};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersCompositionProps {
    composition: Signal<Composition>,
    on_label_select: EventHandler<()>,
    on_label_component_update: EventHandler<Component>,
    on_label_units_update: EventHandler<Units>,
    on_formula_select: EventHandler<()>,
    on_formula_update: EventHandler<String>,
}

#[component]
pub fn FertilizersComposition(props: FertilizersCompositionProps) -> Element {
    let composition = props.composition.read();

    rsx! {
        div {
            class: "fertilizers-composition",

            div {
                class: "fertilizers-source",

                div {
                    class: "fertilizers-source__title",
                    "Состав"
                }

                div {
                    class: "fertilizers-source__tabs",

                    button {
                        class: "fertilizers-source__tab",
                        onclick: move |_| props.on_label_select.call(()),
                        "С этикетки"
                    }

                    button {
                        class: "fertilizers-source__tab",
                        onclick: move |_| props.on_formula_select.call(()),
                        "По формуле"
                    }
                }
            }

            div {
                class: "composition__source",

                if let Composition::Label(label) = composition.clone() {
                    FertilizersLabel {
                        label: Signal::new(label),
                        on_label_units_update: props.on_label_units_update,
                        on_label_component_update: props.on_label_component_update,
                    }
                }

                if let Composition::Formula(formula) = composition.clone() {
                    FertilizersFormula {
                        formula: Signal::new(formula.value()),
                        on_formula_update: props.on_formula_update,
                    }
                }
            }
        }
    }
}
