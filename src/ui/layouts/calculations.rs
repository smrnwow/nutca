use crate::controller::Toaster;
use crate::ui::components::layout::{Page, Section};
use crate::ui::components::utils::Divider;
use crate::ui::components::utils::Notifications;
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
pub fn Calculations() -> Element {
    let current_route = use_route::<Route>();

    let mut toaster = use_context::<Signal<Toaster>>();

    let toasts = use_memo(move || toaster.read().list());

    rsx! {
        div {
            class: "layout layout_calculations",

            aside {
                class: "sidebar",

                div {
                    class: "sidebar__switch",

                    Link {
                        class: "layout__link",
                        to: Route::ReferenceMainPage {},
                        "Справка",
                    }

                    Link {
                        class: "layout__link layout__link_active",
                        to: Route::SolutionsMainPage {},
                        "Расчеты",
                    }
                }

                Divider {}

                nav {
                    class: "navigation",

                    Link {
                        class: link_class(&current_route, Route::SolutionsMainPage {}),
                        to: Route::SolutionsMainPage {},
                        "Растворы",
                    }

                    Link {
                        class: link_class(&current_route, Route::MainConcentratesPage {}),
                        to: Route::MainConcentratesPage {},
                        "Концентраты",
                    }

                    Link {
                        class: link_class(&current_route, Route::FertilizersMainPage {}),
                        to: Route::FertilizersMainPage {},
                        "Удобрения",
                    }

                    Link {
                        class: link_class(&current_route, Route::ProfilesMainPage {}),
                        to: Route::ProfilesMainPage {},
                        "Питательные составы",
                    }

                    Link {
                        class: link_class(&current_route, Route::MainWaterAnalysisPage {}),
                        to: Route::MainWaterAnalysisPage {},
                        "Анализы воды",
                    }
                }
            }

            main {
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
}
