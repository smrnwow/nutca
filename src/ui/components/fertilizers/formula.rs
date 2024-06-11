use crate::model::formulas::Formula;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersFormulaProps {
    formula: Memo<Formula>,
    on_formula_update: EventHandler<String>,
}

#[component]
pub fn FertilizersFormula(props: FertilizersFormulaProps) -> Element {
    rsx! {
        label {
            class: "fertilizers-formula",

            "Формула",

            input {
                class: "fertilizers-formula__input",
                r#type: "text",
                value: "{props.formula.read().formulation()}",
                oninput: move |event| props.on_formula_update.call(event.value()),
            }
        }
    }
}
