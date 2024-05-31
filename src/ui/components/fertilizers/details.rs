use crate::model::fertilizers::Fertilizer;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersDetailsProps {
    fertilizer: Signal<Fertilizer>,
    on_name_update: EventHandler<String>,
    on_vendor_update: EventHandler<String>,
}

#[component]
pub fn FertilizersDetails(props: FertilizersDetailsProps) -> Element {
    let fertilizer = props.fertilizer.read();

    rsx! {
            div {
                class: "fertilizers-details",

                label {
                    class: "fertilizers-details__field",

                    "Название: ",

                    input {
                        class: "fertilizers-details__input",
                        r#type: "text",
                        value: fertilizer.name(),
                        oninput: move |event| props.on_name_update.call(event.value()),
                    }
                }

                label {
                    class: "fertilizers-details__field",

                    "Производитель: ",

                    input {
                        class: "fertilizers-details__input",
                        r#type: "text",
                        value: fertilizer.vendor(),
                        oninput: move |event| props.on_vendor_update.call(event.value()),
                    }
                }
            }
    }
}
