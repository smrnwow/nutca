use crate::controller::solutions::Dashboard;
use crate::repository::Storage;
use crate::ui::components::solutions::SolutionsDashboard;
use dioxus::prelude::*;

#[component]
pub fn SolutionsMainPage() -> Element {
    let storage = consume_context::<Signal<Storage>>();

    let mut dashboard = use_signal(|| Dashboard::new(storage));

    let solutions_listing = use_memo(move || dashboard.read().list_solutions());

    rsx! {
        SolutionsDashboard {
            solutions_listing,
            on_search: move |search_query| {
                dashboard.write().search_solution(search_query);
            },
            on_add: move |_| {
                dashboard.read().add_solution();
            },
            on_open: move |solution_id| {
                dashboard.read().open_solution(solution_id);
            },
            on_stock: move |solution_id| {
                dashboard.read().open_stock_solution(solution_id);
            },
            on_delete: move |solution_id| {
                dashboard.write().delete_solution(solution_id);
            },
            on_paginate: move |page_index| {
                dashboard.write().paginate(page_index);
            },
        }
    }
}
