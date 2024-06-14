use super::provider::Provider;
use crate::model::fertilizers::{Fertilizer, FertilizersListing};
use crate::model::formulas::Formula;
use crate::model::labels::{Component, Label, Units};
use rusqlite::params;
use uuid::Uuid;

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
                    id TEXT PRIMARY KEY,
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
            .execute(
                "INSERT INTO fertilizers (id, data) VALUES (?1, ?2)",
                params![fertilizer.id(), data],
            )
            .unwrap();

        self.storage.connection().last_insert_rowid()
    }

    pub fn get(&self, fertilizer_id: String) -> Option<Fertilizer> {
        let query = format!("SELECT * FROM fertilizers WHERE id = \"{fertilizer_id}\"");

        let response = self.storage.connection().prepare(query.as_str());

        match response {
            Ok(mut result) => {
                let fertilizers: Vec<Fertilizer> = result
                    .query_map([], |row| {
                        let data: String = row.get(1).unwrap();

                        Ok(serde_json::from_str::<Fertilizer>(&data)
                            .expect("Failed to deserialize"))
                    })
                    .unwrap()
                    .map(|fertilizer| fertilizer.unwrap())
                    .collect();

                if fertilizers.len() > 0 {
                    return Some(fertilizers.get(0).unwrap().clone());
                }

                None
            }
            Err(error) => {
                println!("fertilizer get error {:#?}", error);

                None
            }
        }
    }

    pub fn update(&self, fertilizer: Fertilizer) {
        let data = serde_json::to_string(&fertilizer).expect("Failed to serialize");

        let query = "UPDATE fertilizers SET data = ?2 WHERE id = ?1";

        let mut statement = self.storage.connection().prepare(query).unwrap();

        let response = statement.execute(params![fertilizer.id(), data]).unwrap();

        println!("response {:#?}", response);
    }

    pub fn delete(&self, fertilizer_id: String) {
        let query = "DELETE FROM fertilizers WHERE id = ?1";

        let mut statement = self.storage.connection().prepare(query).unwrap();

        statement.execute(params![fertilizer_id]).unwrap();
    }

    pub fn list(&self) -> FertilizersListing {
        let statement = self
            .storage
            .connection()
            .prepare("SELECT * FROM fertilizers");

        match statement {
            Ok(mut query) => {
                let fertilizers = query
                    .query_map([], |row| {
                        let data: String = row.get(1).unwrap();

                        Ok(serde_json::from_str::<Fertilizer>(&data)
                            .expect("Failed to deserialize"))
                    })
                    .unwrap()
                    .map(|fertilizer| fertilizer.unwrap())
                    .collect();

                FertilizersListing::new(fertilizers)
            }

            Err(_) => FertilizersListing::new(vec![]),
        }
    }

    fn seed(&self) {
        let fertilizers = vec![
            Fertilizer::build()
                .with_id(Uuid::new_v4().to_string())
                .with_name(String::from("Кальциевая селитра (3-х водная)"))
                .with_formula(Formula::from("Ca(NO3)2*3H20")),
            Fertilizer::build()
                .with_id(Uuid::new_v4().to_string())
                .with_name(String::from("Кальциевая селитра (4-х водная)"))
                .with_formula(Formula::from("Ca(NO3)2*4H20")),
            Fertilizer::build()
                .with_id(Uuid::new_v4().to_string())
                .with_name(String::from("Калиевая селитра"))
                .with_formula(Formula::from("KNO3")),
            Fertilizer::build()
                .with_id(Uuid::new_v4().to_string())
                .with_name(String::from("Аммиачная селитра"))
                .with_formula(Formula::from("NH4NO3")),
            Fertilizer::build()
                .with_id(Uuid::new_v4().to_string())
                .with_name(String::from("Сульфат магния"))
                .with_formula(Formula::from("MgSO4*7H2O")),
            Fertilizer::build()
                .with_id(Uuid::new_v4().to_string())
                .with_name(String::from("Сульфат калия"))
                .with_formula(Formula::from("K2SO4")),
            Fertilizer::build()
                .with_id(Uuid::new_v4().to_string())
                .with_name(String::from("Монофосфат калия"))
                .with_formula(Formula::from("KH2PO4")),
            Fertilizer::build()
                .with_id(Uuid::new_v4().to_string())
                .with_name(String::from("Кристалон цветочный"))
                .with_vendor(String::from("fertika"))
                .with_label(Label::from(
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
                .with_id(Uuid::new_v4().to_string())
                .with_name(String::from("Унифлор микро"))
                .with_label(Label::from(
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
                .with_id(Uuid::new_v4().to_string())
                .with_name(String::from("Хелат железа DTPA"))
                .with_label(Label::from(Units::Percent, vec![Component::Iron(10.)])),
            Fertilizer::build()
                .with_id(Uuid::new_v4().to_string())
                .with_name(String::from("Сульфат марганца"))
                .with_formula(Formula::from("MnSO4*H2O")),
            Fertilizer::build()
                .with_id(Uuid::new_v4().to_string())
                .with_name(String::from("Борная кислота"))
                .with_formula(Formula::from("H3BO3")),
            Fertilizer::build()
                .with_id(Uuid::new_v4().to_string())
                .with_name(String::from("Молибденовая кислота"))
                .with_formula(Formula::from("Na2MoO4*2H2O")),
            Fertilizer::build()
                .with_id(Uuid::new_v4().to_string())
                .with_name(String::from("Сульфат цинка"))
                .with_formula(Formula::from("ZnSO4*7H2O")),
            Fertilizer::build()
                .with_id(Uuid::new_v4().to_string())
                .with_name(String::from("Сульфат меди"))
                .with_formula(Formula::from("CuSO4*5H2O")),
        ];

        for fertilizer in fertilizers {
            self.add(fertilizer);
        }
    }
}
