use crate::controller::reference::TopicId;
use crate::repository::ProfilesListing;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::profiles::ProfilesListingItem;
use crate::ui::components::reference::ReferenceBadge;
use crate::ui::components::utils::icons::SearchIcon;
use crate::ui::components::utils::{
    Banner, Block, Button, Card, Divider, List, Pagination, TextField, Title,
};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ProfilesListingProps {
    profiles_listing: Memo<ProfilesListing>,
    on_search: EventHandler<String>,
    on_add: EventHandler<()>,
    on_open: EventHandler<String>,
    on_use: EventHandler<String>,
    on_delete: EventHandler<String>,
    on_paginate: EventHandler<usize>,
}

#[component]
pub fn ProfilesListing(props: ProfilesListingProps) -> Element {
    let profiles = use_memo(move || props.profiles_listing.read().list());

    rsx! {
        Card {
            Block {
                Row {
                    Title {
                        {TopicId::ProfilesDashboard.title()}
                    }
                }
            }

            Divider {}

            Block {
                Banner {
                    text: "Питательный состав - это набор требований к объему основных питательных элементов, небходимых растениям для роста и развития. Сбалансированный питательный состав - залог успеха.",
                    more_link: "#",
                }
            }

            Block {
                exclude_padding: "top",

                Row {
                    TextField {
                        value: props.profiles_listing.read().search_query(),
                        placeholder: "найти питательный состав",
                        on_input: props.on_search,
                        icon_left: rsx! {
                            SearchIcon {}
                        },
                    }

                    Button {
                        style: "primary",
                        on_click: props.on_add,
                        "Добавить питательный состав",
                    }
                }
            }

            Block {
                exclude_padding: "top",

                Column {
                    List {
                        limit: 10,
                        empty: profiles.read().len() == 0,
                        stub_text: "Сохраненные питательные составы отсутствуют",

                        for profile in profiles.read().clone() {
                            ProfilesListingItem {
                                key: "{profile.id()}",
                                profile,
                                on_open: props.on_open,
                                on_use: props.on_use,
                                on_delete: props.on_delete,
                            }
                        }
                    }

                    Pagination {
                        page_index: props.profiles_listing.read().page_index(),
                        limit: props.profiles_listing.read().limit(),
                        total: props.profiles_listing.read().total(),
                        on_change: props.on_paginate,
                    }
                }
            }
        }
    }
}
