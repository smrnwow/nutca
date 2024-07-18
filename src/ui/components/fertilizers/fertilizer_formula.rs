use crate::model::fertilizers::formulas::Formula;
use crate::ui::components::utils::TextField;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizerFormulaProps {
    formula: Signal<Formula>,
    on_formula_update: EventHandler<String>,
}

#[component]
pub fn FertilizerFormula(props: FertilizerFormulaProps) -> Element {
    rsx! {
        TextField {
            label: "Формула",
            value: props.formula.read().formulation(),
            error: props.formula.read().error(),
            on_input: props.on_formula_update,
        }
    }
}
