use super::layouts::{Calculations, Reference};
use super::pages::concentrates::{
    Create as CreateConcentratePage, Edit as EditConcentratePage, Main as MainConcentratesPage,
};
use super::pages::fertilizers::{FertilizerAddPage, FertilizerEditPage, FertilizersMainPage};
use super::pages::profiles::{ProfileAddPage, ProfileEditPage, ProfilesMainPage};
use super::pages::reference::ReferenceMainPage;
use super::pages::solutions::{SolutionAddPage, SolutionEditPage, SolutionsMainPage};
use super::pages::water_analysis::Main as MainWaterAnalysisPage;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, PartialEq, Debug, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[redirect("", || Route::ReferenceMainPage {})]
    #[layout(Reference)]
        #[nest("/reference")]
            #[route("/intro")]
            ReferenceMainPage {},
        #[end_nest]
    #[end_layout]
    #[layout(Calculations)]
        #[nest("/calculations")]
            #[nest("/solutions")]
                #[route("/")]
                SolutionsMainPage {},
                #[route("/add?:profile_id&:concentrate_id")]
                SolutionAddPage {
                    profile_id: String,
                    concentrate_id: String,
                },
                #[route("/edit/:solution_id")]
                SolutionEditPage { solution_id: String },
            #[end_nest]
            #[nest("/concentrates")]
                #[route("/")]
                MainConcentratesPage {},
                #[route("/create?:solution_id")]
                CreateConcentratePage { solution_id: String },
                #[route("/edit?:concentrate_id")]
                EditConcentratePage { concentrate_id: String },
            #[end_nest]
            #[nest("/fertilizers")]
                #[route("/")]
                FertilizersMainPage {},           
                #[route("/add")]
                FertilizerAddPage {},
                #[route("/edit/:fertilizer_id")]
                FertilizerEditPage { fertilizer_id: String },
            #[end_nest]
            #[nest("/profiles")]
                #[route("/")]
                ProfilesMainPage {},            
                #[route("/add")]
                ProfileAddPage {},
                #[route("/edit/:profile_id")]
                ProfileEditPage { profile_id: String },        
            #[end_nest]
            #[route("/water-analysis")]
            MainWaterAnalysisPage {},
}
