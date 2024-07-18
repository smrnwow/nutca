use crate::repository::ReferenceBrowser;
use crate::ui::components::layout::Column;
use crate::ui::components::reference::ReferenceTip;
use crate::ui::components::utils::Title;
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ReferenceBadgeProps {
    article_id: String,
}

#[component]
pub fn ReferenceBadge(props: ReferenceBadgeProps) -> Element {
    let reference_browser = consume_context::<Signal<ReferenceBrowser>>();

    let topic = use_signal(|| reference_browser.read().summary(&props.article_id));

    rsx! {
        div {
            class: "reference-badge",

            ReferenceTip {
                tooltip_position: "bottom-center",
                tooltip: rsx! {
                    Column {
                        Title {
                            size: "x-small",
                            {topic.read().title()},
                        }

                        Link {
                            to: Route::ReferenceMainPage {},
                            "Ссылка на справку",
                        }
                    }
                },
            },
        }
    }
}
