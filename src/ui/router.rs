use super::pages::concentrates::{
    Create as CreateConcentratePage, Edit as EditConcentratePage, Main as MainConcentratesPage,
};
use super::pages::fertilizers::{FertilizerAddPage, FertilizerEditPage, FertilizersMainPage};
use super::pages::profiles::{ProfileAddPage, ProfileEditPage, ProfilesMainPage};
use super::pages::reference::ReferenceMainPage;
use super::pages::solutions::{SolutionAddPage, SolutionEditPage, SolutionsMainPage};
use super::pages::water_analysis::Main as MainWaterAnalysisPage;
use super::Layout;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, PartialEq, Debug, Clone)]
pub enum Route {
    #[redirect("", || Route::ReferenceMainPage {})]
    #[layout(Layout)]
    #[route("/reference")]
    ReferenceMainPage {},

    #[route("/solutions")]
    SolutionsMainPage {},

    #[route("/solutions/add?:profile_id")]
    SolutionAddPage { profile_id: String },

    #[route("/solutions/edit/:solution_id")]
    SolutionEditPage { solution_id: String },

    #[route("/concentrates")]
    MainConcentratesPage {},

    #[route("/concentrates/create?:solution_id")]
    CreateConcentratePage { solution_id: String },

    #[route("/concentrates/edit?:concentrate_id")]
    EditConcentratePage { concentrate_id: String },

    #[route("/water-analysis")]
    MainWaterAnalysisPage {},

    #[route("/profiles")]
    ProfilesMainPage {},

    #[route("/profiles/add")]
    ProfileAddPage {},

    #[route("/profiles/edit/:profile_id")]
    ProfileEditPage { profile_id: String },

    #[route("/fertilizers")]
    FertilizersMainPage {},

    #[route("/fertilizers/add")]
    FertilizerAddPage {},

    #[route("/fertilizers/edit/:fertilizer_id")]
    FertilizerEditPage { fertilizer_id: String },
}
