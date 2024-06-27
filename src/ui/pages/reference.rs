use crate::ui::components::layout::{Page, Section};
use crate::ui::components::utils::{Block, Card, Divider, Text, Title};
use dioxus::prelude::*;

#[component]
pub fn Reference() -> Element {
    rsx! {
        Page {
            Section {
                Card {
                    Block {
                        Title {
                            text: "Справка",
                        }
                    }

                    Divider {}

                    Block {
                        Title {
                            size: "small",
                            text: "Интерфейс",
                        }

                        Block {
                            Title {
                                size: "small",
                                text: "Растворы",
                            }

                            Block {
                                Title {
                                    size: "small",
                                    text: "Список растворов",
                                }

                                Title {
                                    size: "small",
                                    text: "Редактор раствора",
                                }

                                Block {
                                    Title {
                                        size: "x-small",
                                        text: "Выбор профиля питания",
                                    }

                                    Text {
                                        size: "x-small",
                                        "растения потребляют питательные вещества в разных количествах. оптимальные соотношения питательных веществ записаны в профилях питания",
                                    }
                                }

                                Block {
                                    Title {
                                        size: "x-small",
                                        text: "Выбор удобрений",
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
                            text: "Теория",
                        }

                        Block {
                            Title {
                                size: "small",
                                text: "Раствор",
                            }

                            Block {
                                Title {
                                    size: "small",
                                    text: "Электропроводность (EC) раствора",
                                }

                                Title {
                                    size: "small",
                                    text: "Формы азота",
                                }
                            }
                        }

                        Block {
                            Title {
                                size: "small",
                                text: "Профиль питания",
                            }

                            Block {
                                Title {
                                    size: "small",
                                    text: "Состав",
                                }

                                Block {
                                    Title {
                                        size: "small",
                                        text: "Макроэлементы",
                                    }

                                    Block {
                                        Title {
                                            size: "x-small",
                                            text: "Азот (N)",
                                        }

                                        Title {
                                            size: "x-small",
                                            text: "Фосфор (P)",
                                        }

                                        Title {
                                            size: "x-small",
                                            text: "Калий (K)",
                                        }

                                        Title {
                                            size: "x-small",
                                            text: "Кальций (Ca)",
                                        }

                                        Title {
                                            size: "x-small",
                                            text: "Магний (Mg)",
                                        }

                                        Title {
                                            size: "x-small",
                                            text: "Сера (S)",
                                        }
                                    }
                                }

                                Block {
                                    Title {
                                        size: "small",
                                        text: "Формы азота",
                                    }

                                    Block {
                                        Title {
                                            size: "x-small",
                                            text: "Нитратная форма азота (NO3)",
                                        }

                                        Title {
                                            size: "x-small",
                                            text: "Аммонийная форма азота (NH4)",
                                        }
                                    }
                                }

                                Block {
                                    Title {
                                        size: "small",
                                        text: "Микроэлементы",
                                    }

                                    Block {
                                        Title {
                                            size: "x-small",
                                            text: "Железо (Fe)",
                                        }

                                        Title {
                                            size: "x-small",
                                            text: "Марганец (Mn)",
                                        }

                                        Title {
                                            size: "x-small",
                                            text: "Медь (Cu)",
                                        }

                                        Title {
                                            size: "x-small",
                                            text: "Цинк (Zn)",
                                        }

                                        Title {
                                            size: "x-small",
                                            text: "Бор (B)",
                                        }

                                        Title {
                                            size: "x-small",
                                            text: "Молибден (Mo)",
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
