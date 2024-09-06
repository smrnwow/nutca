use crate::controller::concentrates::SolutionsBrowser;
use crate::ui::components::layout::Row;
use crate::ui::components::utils::Select;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SolutionSelectProps {
    solutions_browser: Memo<SolutionsBrowser>,
    on_solution_search: EventHandler<String>,
    on_solution_change: EventHandler<String>,
}

#[component]
pub fn SolutionSelect(props: SolutionSelectProps) -> Element {
    let value = use_memo(move || props.solutions_browser.read().value());

    rsx! {
        Row {
            Select {
                label: "Раствор",
                placeholder: "выбрать раствор",
                value,
                options: props.solutions_browser.read().options(),
                on_search: props.on_solution_search,
                on_change: props.on_solution_change,
            }
        }
    }
}
