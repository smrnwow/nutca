use crate::model::formulas::Formula;
use crate::ui::components::utils::TextField;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersFormulaProps {
    formula: Memo<Formula>,
    on_formula_update: EventHandler<String>,
}

#[component]
pub fn FertilizersFormula(props: FertilizersFormulaProps) -> Element {
    rsx! {
        TextField {
            value: props.formula.read().formulation(),
            placeholder: "Формула",
            on_input: move |event| props.on_formula_update.call(event),
        }
    }
}
