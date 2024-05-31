use crate::model::chemistry::NitrogenForm;
use crate::ui::components::NutrientValue;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct NitrogenFormValueProps {
    nitrogen_form: NitrogenForm,
    on_update: EventHandler<NitrogenForm>,
}

#[component]
pub fn NitrogenFormValue(props: NitrogenFormValueProps) -> Element {
    rsx! {
        NutrientValue {
            symbol: props.nitrogen_form.symbol(),
            value: props.nitrogen_form.value(),
            on_change: move |value| props.on_update.call(props.nitrogen_form.new(value)),
        }
    }
}
