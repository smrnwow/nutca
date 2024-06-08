use dioxus::prelude::*;
use dioxus_router::prelude::*;

use super::pages::fertilizers::{FertilizerEditorPage, FertilizersListingPage};
use super::pages::solutions::SolutionEditorPage;
use super::pages::{Profiles, Reference};
use super::Layout;

#[derive(Routable, PartialEq, Debug, Clone)]
pub enum Route {
    #[redirect("", || Route::Reference {})]
    #[layout(Layout)]
    #[route("/reference")]
    Reference {},
    #[route("/calculation")]
    SolutionEditorPage {},
    #[route("/profiles")]
    Profiles {},
    #[route("/fertilizers")]
    FertilizersListingPage {},
    #[route("/fertilizers/add")]
    FertilizerEditorPage {},
}
