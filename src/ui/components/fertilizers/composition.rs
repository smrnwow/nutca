use super::{FertilizersFormula, FertilizersLabel};
use crate::model::chemistry::NitrogenForm;
use crate::model::fertilizers::{Composition, Fertilizer};
use crate::model::labels::{Component, Units};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersCompositionProps {
    fertilizer: Signal<Fertilizer>,
    on_label_select: EventHandler<()>,
    on_label_component_update: EventHandler<Component>,
    on_label_nitrogen_form_update: EventHandler<NitrogenForm>,
    on_label_units_update: EventHandler<Units>,
    on_formula_select: EventHandler<()>,
    on_formula_update: EventHandler<String>,
}

#[component]
pub fn FertilizersComposition(props: FertilizersCompositionProps) -> Element {
    let fertilizer = props.fertilizer.read();

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

                if let Composition::Label(label) = fertilizer.composition().clone() {
                    FertilizersLabel {
                        label: Signal::new(label),
                        on_label_units_update: props.on_label_units_update,
                        on_label_component_update: props.on_label_component_update,
                        on_label_nitrogen_form_update: props.on_label_nitrogen_form_update,
                    }
                }

                if let Composition::Formula(formula) = fertilizer.composition().clone() {
                    FertilizersFormula {
                        formula: Signal::new(formula.formulation()),
                        on_formula_update: props.on_formula_update,
                    }
                }
            }
        }
    }
}
