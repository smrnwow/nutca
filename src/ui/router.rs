use dioxus::prelude::*;
use dioxus_router::prelude::*;

use super::pages::fertilizers::{FertilizerAddPage, FertilizerEditPage, FertilizersListingPage};
use super::pages::profiles::{ProfileAddPage, ProfileEditPage, ProfilesListingPage};
use super::pages::reference::ReferenceMainPage;
use super::pages::solutions::{
    SolutionAddPage, SolutionEditPage, SolutionsListingPage, StockSolutionPage,
};
use super::Layout;

#[derive(Routable, PartialEq, Debug, Clone)]
pub enum Route {
    #[redirect("", || Route::ReferenceMainPage {})]
    #[layout(Layout)]
    #[route("/reference")]
    ReferenceMainPage {},

    #[route("/solutions")]
    SolutionsListingPage {},

    #[route("/solutions/add?:profile_id")]
    SolutionAddPage { profile_id: String },

    #[route("/solutions/edit/:solution_id")]
    SolutionEditPage { solution_id: String },

    #[route("/solutions/stock?:solution_id")]
    StockSolutionPage { solution_id: String },

    #[route("/profiles")]
    ProfilesListingPage {},

    #[route("/profiles/add")]
    ProfileAddPage {},

    #[route("/profiles/edit/:profile_id")]
    ProfileEditPage { profile_id: String },

    #[route("/fertilizers")]
    FertilizersListingPage {},

    #[route("/fertilizers/add")]
    FertilizerAddPage {},

    #[route("/fertilizers/edit/:fertilizer_id")]
    FertilizerEditPage { fertilizer_id: String },
}
