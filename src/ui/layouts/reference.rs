use crate::ui::components::layout::{Page, Section};
use crate::ui::components::utils::Divider;
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

            aside {
                class: "sidebar",

                div {
                    class: "sidebar__switch",

                    Link {
                        class: "layout__link layout__link_active",
                        to: Route::ReferenceMainPage {},
                        "Справка",
                    }

                    Link {
                        class: "layout__link",
                        to: Route::SolutionsMainPage {},
                        "Расчеты",
                    }
                }

                Divider {}

                nav {
                    class: "navigation",

                    Link {
                        class: link_class(&current_route, Route::ReferenceMainPage {}),
                        to: Route::ReferenceMainPage {},
                        "Введение",
                    }
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
