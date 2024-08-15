use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::LaunchBuilder;

mod controller;
mod model;
mod repository;
mod ui;

fn main() {
    LaunchBuilder::desktop()
        .with_cfg(
            Config::new()
                .with_window(
                    WindowBuilder::new()
                        .with_title("nutca - nutrient calculator for hydroponics")
                        .with_resizable(true),
                )
                .with_disable_context_menu(false)
                .with_menu(None),
        )
        .launch(ui::App)
}
