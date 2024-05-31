use crate::storage::FertilizersStorage;
use crate::ui::components::NutrientValue;
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn FertilizersIndexPage() -> Element {
    let fertilizers_storage = consume_context::<Signal<FertilizersStorage>>();

    let fertilizers_list = fertilizers_storage.read().list();

    rsx! {
        div {
            class: "fertilizers-index",

            div {
                class: "fertilizers__summary",

                div {
                     class: "fertilizers__count",

                     p {
                         class: "fertlizers__number",
                         "{fertilizers_list.len()} удобрений"
                     }
                }

                button {
                     class: "add-fertilizer-button",
                     onclick: |_| {
                         navigator().push(Route::FertilizersAddPage {});
                     },
                     "+"
                     /*
                     img {
                         class: "add-fertilizer-button__icon",
                         src: "{icon}"
                     }
                     */
                }
             }

             div {
                 class: "fertilizers__list",

                 for fertilizer in fertilizers_list {
                     div {
                         class: "fertilizers__item",

                         p {
                             class: "fertilizers__name",
                             "{fertilizer.name()}"
                         }

                         div {
                             class: "fertilizers__nutrients",

                             for nutrient in fertilizer.nutrients() {
                                 NutrientValue {
                                     symbol: nutrient.symbol(),
                                     value: nutrient.value(),
                                 }
                             }
                         }
                     }
                 }
            }
        }
    }
}
