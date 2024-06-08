use super::provider::Provider;
use crate::model::fertilizers::Fertilizer;
use crate::model::formulas::Formula;
use crate::model::labels::{Component, Label, Units};
use rusqlite::params;

#[derive(Debug)]
pub struct FertilizersStorage {
    storage: Provider,
}

impl FertilizersStorage {
    pub fn new() -> Self {
        let storage = Provider::new();

        storage
            .connection()
            .execute(
                "CREATE TABLE fertilizers (
                    id INTEGER PRIMARY KEY autoincrement,
                    data TEXT NOT NULL
                )",
                (),
            )
            .unwrap();

        let fertilizers_storage = Self { storage };

        fertilizers_storage.seed();

        fertilizers_storage
    }

    pub fn add(&self, fertilizer: Fertilizer) -> i64 {
        let data = serde_json::to_string(&fertilizer).expect("Failed to serialize");

        self.storage
            .connection()
            .execute("INSERT INTO fertilizers (data) VALUES (?1)", params![data])
            .unwrap();

        self.storage.connection().last_insert_rowid()
    }

    pub fn list(&self) -> Vec<Fertilizer> {
        let query = self
            .storage
            .connection()
            .prepare("SELECT * FROM fertilizers");

        if query.is_ok() {
            query
                .unwrap()
                .query_map([], |row| {
                    // println!("{:#?}", row);

                    let data: String = row.get(1).unwrap();

                    Ok(serde_json::from_str::<Fertilizer>(&data).expect("Failed to deserialize"))
                })
                .unwrap()
                .map(|fertilizer| fertilizer.unwrap())
                .collect()
        } else {
            vec![]
        }
    }

    fn seed(&self) {
        let fertilizers = vec![
            Fertilizer::build()
                .set_name(String::from("кальциевая селитра"))
                .set_formula(Formula::from("Ca(NO3)2")),
            Fertilizer::build()
                .set_name(String::from("калиевая селитра"))
                .set_formula(Formula::from("KNO3")),
            Fertilizer::build()
                .set_name(String::from("аммиачная селитра"))
                .set_formula(Formula::from("NH4NO3")),
            Fertilizer::build()
                .set_name(String::from("сульфат магния"))
                .set_formula(Formula::from("MgSO4*7H2O")),
            Fertilizer::build()
                .set_name(String::from("сульфат калия"))
                .set_formula(Formula::from("K2SO4")),
            Fertilizer::build()
                .set_name(String::from("монофосфат калия"))
                .set_formula(Formula::from("KH2PO4")),
            Fertilizer::build()
                .set_name(String::from("кристалон цветочный"))
                .set_vendor(String::from("fertika"))
                .set_label(Label::from(
                    Units::Percent,
                    vec![
                        Component::Nitrogen(19.),
                        Component::PhosphorPentoxide(6.),
                        Component::PotassiumOxide(20.),
                        Component::MagnesiumOxide(3.),
                        Component::Sulfur(3.),
                        Component::SulfurTrioxide(7.5),
                        Component::Iron(0.07),
                        Component::Manganese(0.04),
                        Component::Copper(0.01),
                        Component::Zinc(0.025),
                        Component::Boron(0.025),
                        Component::Molybdenum(0.004),
                    ],
                )),
            Fertilizer::build()
                .set_name(String::from("унифлор микро"))
                .set_label(Label::from(
                    Units::WeightVolume,
                    vec![
                        Component::Magnesium(15000.),
                        Component::Iron(3200.),
                        Component::Manganese(1600.),
                        Component::Boron(1200.),
                        Component::Zinc(360.),
                        Component::Copper(320.),
                        Component::Molybdenum(102.),
                    ],
                )),
            Fertilizer::build()
                .set_name(String::from("хелат железа"))
                .set_formula(Formula::from("C14H18N3O10Fe(NH4)2")),
            Fertilizer::build()
                .set_name(String::from("сульфат марганца"))
                .set_formula(Formula::from("MnSO4*H2O")),
            Fertilizer::build()
                .set_name(String::from("борная кислота"))
                .set_formula(Formula::from("H3BO3")),
            Fertilizer::build()
                .set_name(String::from("молибденовая кислота"))
                .set_formula(Formula::from("Na2MoO4*2H2O")),
            Fertilizer::build()
                .set_name(String::from("сульфат цинка"))
                .set_formula(Formula::from("ZnSO4*7H2O")),
            Fertilizer::build()
                .set_name(String::from("сульфат меди"))
                .set_formula(Formula::from("CuSO4*5H2O")),
        ];

        for fertilizer in fertilizers {
            self.add(fertilizer);
        }
    }
}
