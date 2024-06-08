use dioxus::prelude::*;
use dioxus_router::prelude::*;

use super::router::Route;

fn link_class(current_route: &Route, path: Route) -> &str {
    if current_route.is_child_of(&path) {
        "navigation__link navigation__link_active"
    } else {
        "navigation__link"
    }
}

#[component]
pub fn Layout() -> Element {
    let current_route = use_route::<Route>();

    rsx! {
        header {
            class: "header",

            style { "{include_str!(\"./styles/index.css\")}" }

            nav {
                class: "navigation",

                Link {
                    class: link_class(&current_route, Route::Reference {}),
                    to: Route::Reference {},
                    "Справка",
                }

                Link {
                    class: link_class(&current_route, Route::SolutionEditorPage {}),
                    to: Route::SolutionEditorPage {},
                    "Растворы",
                }

                Link {
                    class: link_class(&current_route, Route::Profiles {}),
                    to: Route::Profiles {},
                    "Профили",
                }

                Link {
                    class: link_class(&current_route, Route::FertilizersListingPage {}),
                    to: Route::FertilizersListingPage {},
                    "Удобрения",
                }
            }
        }

        main {
            class: "content",
            Outlet::<Route> { }
        }
    }
}
