use crate::ui::components::utils::icons::{Check, Close, Exclamation};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct NotificationsProps {
    notifications: Memo<Vec<(String, String, String)>>,
    on_close: EventHandler<String>,
}

#[component]
pub fn Notifications(props: NotificationsProps) -> Element {
    rsx! {
        div {
            class: "notifications",

            for (id, title, state) in props.notifications.read().clone() {
                div {
                    key: "{id}",
                    class: "notifications__item",

                    div {
                        class: "notifications-item__state notifications-item__state_{state}",

                        div {
                            class: "notifications-item__icon",

                            if state == "success" {
                                Check {}
                            } else {
                                Exclamation {}
                            }
                        }
                    }

                    div {
                        class: "notifications-item__body",
                        {title},
                    }

                    div {
                        class: "notifications-item__control",
                        onclick: move |_| props.on_close.call(id.clone()),
                        Close {},
                    }
                }
            }
        }
    }
}
