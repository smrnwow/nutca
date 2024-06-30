use crate::ui::components::utils::Select;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct UnitsSelectProps {
    value: Option<String>,
    on_change: EventHandler<String>,
}

#[component]
pub fn UnitsSelect(props: UnitsSelectProps) -> Element {
    let mut units_selected = use_signal(|| props.value.unwrap_or(String::from("litres")));

    let units_options = use_signal(|| {
        vec![
            (String::from("litres"), String::from("литры")),
            (String::from("millilitres"), String::from("миллилитры")),
            (String::from("gallons"), String::from("галлоны")),
        ]
    });

    let value = use_memo(move || {
        let selected = units_selected.read().clone();

        let units_options = units_options.read();

        units_options
            .iter()
            .cloned()
            .find(|units_option| units_option.0 == selected)
            .unwrap_or(units_options[0].clone())
    });

    rsx! {
        Select {
            value,
            options: units_options.read().clone(),
            on_change: move |units| {
                *units_selected.write() = units;
            },
        }
    }
}
