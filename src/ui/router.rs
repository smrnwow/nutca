use dioxus::prelude::*;
use dioxus_router::prelude::*;

use super::pages::fertilizers::{FertilizersAddPage, FertilizersIndexPage};
use super::pages::{Calculation, Reference};
use super::Layout;

#[derive(Routable, PartialEq, Debug, Clone)]
pub enum Route {
    #[redirect("", || Route::Reference {})]
    #[layout(Layout)]
    #[route("/reference")]
    Reference {},
    #[route("/calculation")]
    Calculation {},
    #[route("/fertilizers")]
    FertilizersIndexPage {},
    #[route("/fertilizers/add")]
    FertilizersAddPage {},
}
