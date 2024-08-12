use crate::ui::components::layout::Column;
use crate::ui::components::utils::{Block, Card, Divider, Title};
use dioxus::prelude::*;

#[component]
pub fn ReferenceMainPage() -> Element {
    rsx! {
        Card {
            Block {
                Title {
                    "Справка",
                }
            }

            Divider {}

            Block {
                Column {}
            }
        }
    }
}
