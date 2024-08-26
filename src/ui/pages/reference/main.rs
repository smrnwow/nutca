use crate::ui::components::layout::Column;
use crate::ui::components::utils::{Block, Card, Divider, Text, Title};
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
                Column {
                    Column {
                        gap: "x-small",

                        Title {
                            size: "small",
                            "Питательные составы",
                        }

                        Text {
                            size: "x-small",
                            "В питательном составе собраны потребности растения в основных питательных элементах.",
                        }
                    }

                    Column {
                        gap: "x-small",

                        Title {
                            size: "small",
                            "Удобрения",
                        }

                        Text {
                            size: "x-small",
                            "Удобрения - это источник основных питательных элементов. Набор качественных и совместимых удобрений обеспечит потребность растений в питании.",
                        }
                    }

                    Column {
                        gap: "x-small",

                        Title {
                            size: "small",
                            "Растворы",
                        }

                        Text {
                            size: "x-small",
                            "Раствор - это рецепт в котором записан питательный состав, который требуется растению и набор удобрений с рассчитанным для него объемом.",
                        }
                    }
                }
            }
        }
    }
}
