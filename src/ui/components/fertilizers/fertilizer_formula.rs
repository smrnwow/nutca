use crate::model::fertilizers::Fertilizer;
use crate::model::formulas::Formula;
use crate::ui::components::utils::TextField;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizerFormulaProps {
    fertilizer: Memo<Fertilizer>,
    formula: Memo<Formula>,
    on_formula_update: EventHandler<String>,
}

#[component]
pub fn FertilizerFormula(props: FertilizerFormulaProps) -> Element {
    rsx! {
        TextField {
            value: props.formula.read().formulation(),
            label: "Формула",
            on_input: move |event| props.on_formula_update.call(event),
        }
    }
}
