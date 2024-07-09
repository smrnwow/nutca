use crate::model::NotificationContainer;
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

    let mut notifications_container = use_context::<Signal<NotificationContainer>>();

    let notifications = use_memo(move || notifications_container.read().list());

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
                    class: link_class(&current_route, Route::SolutionsListingPage {}),
                    to: Route::SolutionsListingPage {},
                    "Растворы",
                }

                Link {
                    class: link_class(&current_route, Route::ProfilesListingPage {}),
                    to: Route::ProfilesListingPage {},
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
            Outlet::<Route> { },
            Notifications {
                notifications,
                on_close: move |notification_id| {
                    notifications_container.write().remove(notification_id);
                },
            },
        }
    }
}
