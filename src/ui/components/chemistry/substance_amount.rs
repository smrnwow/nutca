use crate::ui::components::utils::FloatField;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SubstanceAmountProps {
    amount: f64,
    is_liquid: bool,
    on_amount_update: EventHandler<f64>,
}

#[component]
pub fn SubstanceAmount(props: SubstanceAmountProps) -> Element {
    let aggregation_state = AggregateState::new(props.is_liquid, props.amount);

    rsx! {
        div {
            class: "fertilizers-set-item__amount",

            FloatField {
                value: aggregation_state.value(),
                on_change: move |value| {
                    props.on_amount_update.call(value);
                },
            }

            span {
                class: "fertilizers-set-item__units",
                {aggregation_state.units()},
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct AmountOrder(isize);

impl AmountOrder {
    pub fn from_amount(amount: f64) -> Self {
        if amount == 0.0 {
            return Self(0);
        }

        let order = amount.abs().log10().floor() as isize;

        return Self(order);
    }

    pub fn value(&self) -> isize {
        self.0
    }
}

trait Units {
    fn from_amount_order(amount_order: AmountOrder) -> Self;
    fn short_name(&self) -> &str;
    fn convert(&self, amount: f64) -> f64;
}

#[derive(Debug, PartialEq)]
enum SolidUnits {
    Kilograms,
    Grams,
    Milligrams,
}

#[derive(Debug, PartialEq)]
enum LiquidUnits {
    Litres,
    Millilitres,
    Microlitres,
}

impl Units for SolidUnits {
    fn from_amount_order(amount_order: AmountOrder) -> Self {
        match amount_order.value() {
            ..-2 => Self::Milligrams,
            -2..3 => Self::Grams,
            3.. => Self::Kilograms,
        }
    }

    fn short_name(&self) -> &str {
        match self {
            Self::Kilograms => "кг",
            Self::Grams => "г",
            Self::Milligrams => "мг",
        }
    }

    fn convert(&self, amount: f64) -> f64 {
        match self {
            Self::Kilograms => amount / 1000.0,
            Self::Grams => amount,
            Self::Milligrams => amount * 10000.0,
        }
    }
}

impl Units for LiquidUnits {
    fn from_amount_order(amount_order: AmountOrder) -> Self {
        match amount_order.value() {
            ..-2 => Self::Microlitres,
            -2..3 => Self::Millilitres,
            3.. => Self::Litres,
        }
    }

    fn short_name(&self) -> &str {
        match self {
            Self::Litres => "л",
            Self::Millilitres => "мл",
            Self::Microlitres => "мкл",
        }
    }

    fn convert(&self, amount: f64) -> f64 {
        match self {
            Self::Litres => amount / 1000.0,
            Self::Millilitres => amount,
            Self::Microlitres => amount * 10000.0,
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum AggregateState {
    Liquid(f64),
    Solid(f64),
}

impl AggregateState {
    pub fn new(is_liquid: bool, amount: f64) -> Self {
        match is_liquid {
            true => Self::Liquid(amount),
            false => Self::Solid(amount),
        }
    }

    pub fn units(&self) -> String {
        match self {
            Self::Liquid(amount) => {
                let units = LiquidUnits::from_amount_order(AmountOrder::from_amount(*amount));

                units.short_name().to_string()
            }

            Self::Solid(amount) => {
                let units = SolidUnits::from_amount_order(AmountOrder::from_amount(*amount));

                units.short_name().to_string()
            }
        }
    }

    pub fn value(&self) -> f64 {
        match self {
            Self::Liquid(amount) => {
                let units = LiquidUnits::from_amount_order(AmountOrder::from_amount(*amount));

                units.convert(*amount)
            }

            Self::Solid(amount) => {
                let units = SolidUnits::from_amount_order(AmountOrder::from_amount(*amount));

                units.convert(*amount)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AggregateState, AmountOrder, LiquidUnits, SolidUnits, Units};

    #[test]
    fn correctly_selects_amount_order() {
        assert_eq!(AmountOrder(3), AmountOrder::from_amount(3000.0));

        assert_eq!(AmountOrder(2), AmountOrder::from_amount(300.0));

        assert_eq!(AmountOrder(0), AmountOrder::from_amount(3.0));

        assert_eq!(AmountOrder(0), AmountOrder::from_amount(0.0));

        assert_eq!(AmountOrder(-3), AmountOrder::from_amount(0.003));

        assert_eq!(AmountOrder(-6), AmountOrder::from_amount(0.000003));
    }

    #[test]
    fn correctly_selects_appropriate_solid_units() {
        assert_eq!(
            SolidUnits::Grams,
            SolidUnits::from_amount_order(AmountOrder::from_amount(3.0))
        );

        assert_eq!(
            SolidUnits::Grams,
            SolidUnits::from_amount_order(AmountOrder::from_amount(300.0))
        );

        assert_eq!(
            SolidUnits::Kilograms,
            SolidUnits::from_amount_order(AmountOrder::from_amount(3000.0))
        );

        assert_eq!(
            SolidUnits::Milligrams,
            SolidUnits::from_amount_order(AmountOrder::from_amount(0.003))
        );
    }

    #[test]
    fn correctly_selects_appropriate_liquid_units() {
        assert_eq!(
            LiquidUnits::Millilitres,
            LiquidUnits::from_amount_order(AmountOrder::from_amount(3.0))
        );

        assert_eq!(
            LiquidUnits::Millilitres,
            LiquidUnits::from_amount_order(AmountOrder::from_amount(300.0))
        );

        assert_eq!(
            LiquidUnits::Litres,
            LiquidUnits::from_amount_order(AmountOrder::from_amount(3000.0))
        );

        assert_eq!(
            LiquidUnits::Microlitres,
            LiquidUnits::from_amount_order(AmountOrder::from_amount(0.003))
        );
    }

    #[test]
    fn correctly_converts_amount() {
        let aggregate_state = AggregateState::new(false, 59.0);

        assert_eq!(
            SolidUnits::Grams.short_name().to_string(),
            aggregate_state.units()
        );
    }
}
