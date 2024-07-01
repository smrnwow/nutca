use crate::model::reference::Browser;
use crate::ui::components::layout::Column;
use crate::ui::components::utils::{Reference, Text, Title};
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ReferencePreviewProps {
    article_id: String,
    show_reference: Signal<bool>,
}

#[component]
pub fn ReferencePreview(props: ReferencePreviewProps) -> Element {
    let reference_browser = consume_context::<Signal<Browser>>();

    let article = use_signal(|| reference_browser.read().summary(&props.article_id));

    rsx! {
        Reference {
            display: Signal::new(false),
            style: "badge",
            tooltip: rsx! {
                Column {
                    Title {
                        size: "x-small",
                        {article.read().title()},
                    }

                    Text {
                        size: "x-small",
                        {article.read().text()},
                    }

                    Link {
                        to: Route::ReferenceArticlePage {
                            article_id: article.read().id(),
                        },
                        "Ссылка на справку",
                    }
                }
            },
            tooltip_position: "top-center",
        },
    }
}
