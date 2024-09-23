use crate::ui::components::layout::{Page, Row, Section};
use crate::ui::components::utils::{Block, Divider};
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

fn link_class(current_route: &Route, path: Route) -> &str {
    if current_route.is_child_of(&path) {
        "navigation__link navigation__link_active"
    } else {
        "navigation__link"
    }
}

#[component]
pub fn Reference() -> Element {
    let current_route = use_route::<Route>();

    rsx! {
        div {
            class: "layout layout_reference",

            nav {
                class: "navigation",

                Block {
                    Row {
                        Link {
                            to: Route::ReferenceMainPage {},
                            "Справка",
                        }

                        Link {
                            to: Route::SolutionsMainPage {},
                            "Расчеты",
                        }
                    }
                }

                Divider {}

                Link {
                    class: link_class(&current_route, Route::ReferenceMainPage {}),
                    to: Route::ReferenceMainPage {},
                    "Введение",
                }
            }

            main {
                Page {
                    Section {
                        Outlet::<Route> { },
                    }
                }
            }
        }
    }
}
