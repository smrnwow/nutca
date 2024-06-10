use dioxus::prelude::*;
use dioxus_router::prelude::*;

use super::pages::fertilizers::{FertilizerEditorPage, FertilizersListingPage};
use super::pages::profiles::{ProfileEditorPage, ProfilesListingPage};
use super::pages::solutions::{SolutionAddPage, SolutionEditPage, SolutionsListingPage};
use super::pages::Reference;
use super::Layout;

#[derive(Routable, PartialEq, Debug, Clone)]
pub enum Route {
    #[redirect("", || Route::Reference {})]
    #[layout(Layout)]
    #[route("/reference")]
    Reference {},

    #[route("/solutions/listing")]
    SolutionsListingPage {},

    #[route("/solutions/add")]
    SolutionAddPage {},

    #[route("/solutions/edit/:solution_id")]
    SolutionEditPage { solution_id: String },

    #[route("/profiles/listing")]
    ProfilesListingPage {},

    #[route("/profiles/editor")]
    ProfileEditorPage {},

    #[route("/fertilizers/listing")]
    FertilizersListingPage {},

    #[route("/fertilizers/editor")]
    FertilizerEditorPage {},
}
