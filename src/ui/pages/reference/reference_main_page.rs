use crate::ui::components::layout::{Page, Section};
use crate::ui::components::utils::{Block, Card, Divider, Text, Title};
use dioxus::prelude::*;

#[component]
pub fn ReferenceMainPage() -> Element {
    rsx! {
        Page {
            Section {
                Card {
                    Block {
                        Title {
                            "Справка",
                        }
                    }

                    Divider {}

                    Block {
                        Title {
                            size: "small",
                            "Интерфейс",
                        }

                        Block {
                            Title {
                                size: "small",
                                "Растворы",
                            }

                            Block {
                                Title {
                                    size: "small",
                                    "Список растворов",
                                }

                                Title {
                                    size: "small",
                                    "Редактор раствора",
                                }

                                Block {
                                    Title {
                                        size: "x-small",
                                        "Выбор профиля питания",
                                    }

                                    Text {
                                        size: "x-small",
                                        "растения потребляют питательные вещества в разных количествах. оптимальные соотношения питательных веществ записаны в профилях питания",
                                    }
                                }

                                Block {
                                    Title {
                                        size: "x-small",
                                        "Выбор удобрений",
                                    }

                                    Text {
                                        size: "x-small",
                                        "удобрения содержат питательные вещества, которые нужны растениям",
                                    }
                                }
                            }
                        }
                    }

                    Block {
                        Title {
                            id: "theory",
                            size: "small",
                            "Теория",
                        }

                        Block {
                            Title {
                                size: "small",
                                "Раствор",
                            }

                            Block {
                                Title {
                                    size: "small",
                                    "Электропроводность (EC) раствора",
                                }

                                Title {
                                    size: "small",
                                    "Формы азота",
                                }
                            }
                        }

                        Block {
                            Title {
                                size: "small",
                                "Профиль питания",
                            }

                            Block {
                                Title {
                                    size: "small",
                                    "Состав",
                                }

                                Block {
                                    Title {
                                        size: "small",
                                        "Макроэлементы",
                                    }

                                    Block {
                                        Title {
                                            size: "x-small",
                                            "Азот (N)",
                                        }

                                        Title {
                                            size: "x-small",
                                            "Фосфор (P)",
                                        }

                                        Title {
                                            size: "x-small",
                                            "Калий (K)",
                                        }

                                        Title {
                                            size: "x-small",
                                            "Кальций (Ca)",
                                        }

                                        Title {
                                            size: "x-small",
                                            "Магний (Mg)",
                                        }

                                        Title {
                                            size: "x-small",
                                            "Сера (S)",
                                        }
                                    }
                                }

                                Block {
                                    Title {
                                        size: "small",
                                        "Формы азота",
                                    }

                                    Block {
                                        Title {
                                            size: "x-small",
                                            "Нитратная форма азота (NO3)",
                                        }

                                        Title {
                                            size: "x-small",
                                            "Аммонийная форма азота (NH4)",
                                        }
                                    }
                                }

                                Block {
                                    Title {
                                        size: "small",
                                        "Микроэлементы",
                                    }

                                    Block {
                                        Title {
                                            size: "x-small",
                                            "Железо (Fe)",
                                        }

                                        Title {
                                            size: "x-small",
                                            "Марганец (Mn)",
                                        }

                                        Title {
                                            size: "x-small",
                                            "Медь (Cu)",
                                        }

                                        Title {
                                            size: "x-small",
                                            "Цинк (Zn)",
                                        }

                                        Title {
                                            size: "x-small",
                                            "Бор (B)",
                                        }

                                        Title {
                                            size: "x-small",
                                            "Молибден (Mo)",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
