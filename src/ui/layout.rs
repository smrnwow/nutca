use crate::controller::Toaster;
use crate::ui::components::layout::{Page, Section};
use crate::ui::components::utils::Notifications;
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

    let mut toaster = use_context::<Signal<Toaster>>();

    let toasts = use_memo(move || toaster.read().list());

    rsx! {
        header {
            class: "header",

            style { "{include_str!(\"./styles/index.css\")}" }

            nav {
                class: "navigation",

                Link {
                    class: link_class(&current_route, Route::ReferenceMainPage {}),
                    to: Route::ReferenceMainPage {},
                    "Справка",
                }

                Link {
                    class: link_class(&current_route, Route::ProfilesMainPage {}),
                    to: Route::ProfilesMainPage {},
                    "Питательные составы",
                }

                Link {
                    class: link_class(&current_route, Route::FertilizersMainPage {}),
                    to: Route::FertilizersMainPage {},
                    "Удобрения",
                }

                Link {
                    class: link_class(&current_route, Route::SolutionsMainPage {}),
                    to: Route::SolutionsMainPage {},
                    "Растворы",
                }
            }
        }

        main {
            class: "content",

            Page {
                Section {
                    Outlet::<Route> { },
                }
            }

            Notifications {
                notifications: toasts,
                on_close: move |notification_id| {
                    toaster.write().remove(notification_id);
                },
            },
        }
    }
}
