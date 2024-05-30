use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersDetailsProps {
    name: Signal<String>,
    on_name_update: EventHandler<String>,
    vendor: Signal<String>,
    on_vendor_update: EventHandler<String>,
}

#[component]
pub fn FertilizersDetails(props: FertilizersDetailsProps) -> Element {
    rsx! {
            div {
                class: "fertilizers-details",

                label {
                    class: "fertilizers-details__field",

                    "Название: ",

                    input {
                        class: "fertilizers-details__input",
                        r#type: "text",
                        value: props.name.read().clone(),
                        oninput: move |event| props.on_name_update.call(event.value()),
                    }
                }

                label {
                    class: "fertilizers-details__field",

                    "Производитель: ",

                    input {
                        class: "fertilizers-details__input",
                        r#type: "text",
                        value: props.vendor.read().clone(),
                        oninput: move |event| props.on_vendor_update.call(event.value()),
                    }
                }
            }
    }
}
