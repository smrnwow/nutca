use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::LaunchBuilder;

mod controller;
mod repository;
mod ui;

fn main() {
    LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(WindowBuilder::new().with_resizable(true)))
        .launch(ui::App)
}
