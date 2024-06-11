use dioxus::prelude::*;
use dioxus_router::prelude::*;

use super::pages::fertilizers::{FertilizerAddPage, FertilizerEditPage, FertilizersListingPage};
use super::pages::profiles::{ProfileAddPage, ProfileEditPage, ProfilesListingPage};
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

    #[route("/profiles/add")]
    ProfileAddPage {},

    #[route("/profiles/edit/:profile_id")]
    ProfileEditPage { profile_id: String },

    #[route("/fertilizers/listing")]
    FertilizersListingPage {},

    #[route("/fertilizers/add")]
    FertilizerAddPage {},

    #[route("/fertilizers/edit/:fertilizer_id")]
    FertilizerEditPage { fertilizer_id: String },
}
