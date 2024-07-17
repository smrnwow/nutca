use crate::repository::ArticlesBrowser;
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
    let reference_browser = consume_context::<Signal<ArticlesBrowser>>();

    let article = use_signal(|| reference_browser.read().summary(&props.article_id));

    rsx! {
        div {
            class: "reference-badge",

            ReferenceTip {
                tooltip_position: "bottom-center",
                tooltip: rsx! {
                    Column {
                        Title {
                            size: "x-small",
                            {article.read().title()},
                        }

                        Link {
                            to: Route::ReferenceArticlePage {
                                article_id: article.read().id(),
                            },
                            "Ссылка на справку",
                        }
                    }
                },
            },
        }
    }
}
