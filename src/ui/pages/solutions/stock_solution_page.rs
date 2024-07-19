use crate::controller::solutions::StockSolutionEditor;
use crate::repository::Storage;
use crate::ui::components::solutions::StockSolutionEditor;
use dioxus::prelude::*;

#[component]
pub fn StockSolutionPage(solution_id: String) -> Element {
    let storage = consume_context::<Signal<Storage>>();

    let mut stock_solution_editor = use_signal(|| StockSolutionEditor::new(storage, solution_id));

    let solutions_listing = use_memo(move || stock_solution_editor.read().list_solutions());

    let solution = use_memo(move || stock_solution_editor.read().solution().clone());

    let concentration = use_memo(move || stock_solution_editor.read().concentration());

    let volume = use_memo(move || stock_solution_editor.read().volume());

    let part_a = use_memo(move || stock_solution_editor.read().part_a());

    let part_b = use_memo(move || stock_solution_editor.read().part_b());

    rsx! {
        StockSolutionEditor {
            solution,
            solutions_listing,
            concentration,
            volume,
            part_a,
            part_b,
            on_solution_search: move |search_query| {
                stock_solution_editor.write().search_solution(search_query);
            },
            on_solution_change: move |solution_id| {
                stock_solution_editor.write().change_solution(solution_id);
            },
            on_concentration_change: move |value| {
                stock_solution_editor.write().change_concentration(value);
            },
            on_volume_change: move |volume| {
                stock_solution_editor.write().change_volume(volume);
            },
        }
    }
}
