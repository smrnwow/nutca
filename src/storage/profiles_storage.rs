use super::provider::Provider;
use crate::model::chemistry::{NitrogenForm, NutrientAmount};
use crate::model::profiles::{Component, Profile};
use rusqlite::params;

#[derive(Debug)]
pub struct ProfilesStorage {
    storage: Provider,
}

impl ProfilesStorage {
    pub fn new() -> Self {
        let storage = Provider::new();

        storage
            .connection()
            .execute(
                "CREATE TABLE profiles (
                    id INTEGER PRIMARY KEY autoincrement,
                    data TEXT NOT NULL
                )",
                (),
            )
            .unwrap();

        let profiles_storage = Self { storage };

        profiles_storage.seed();

        profiles_storage
    }

    pub fn add(&self, profile: Profile) -> i64 {
        let data = serde_json::to_string(&profile).expect("Failed to serialize");

        self.storage
            .connection()
            .execute("INSERT INTO profiles (data) VALUES (?1)", params![data])
            .unwrap();

        self.storage.connection().last_insert_rowid()
    }

    pub fn list(&self) -> Vec<Profile> {
        let query = self.storage.connection().prepare("SELECT * FROM profiles");

        if query.is_ok() {
            query
                .unwrap()
                .query_map([], |row| {
                    // println!("{:#?}", row);

                    let data: String = row.get(1).unwrap();

                    Ok(serde_json::from_str::<Profile>(&data).expect("Failed to deserialize"))
                })
                .unwrap()
                .map(|profile| profile.unwrap())
                .collect()
        } else {
            vec![]
        }
    }

    fn seed(&self) {
        let profiles = vec![
            Profile::from(
                "grow",
                vec![
                    Component::Nutrient(NutrientAmount::Nitrogen(189.0)),
                    Component::Nutrient(NutrientAmount::Phosphorus(39.0)),
                    Component::Nutrient(NutrientAmount::Potassium(341.0)),
                    Component::Nutrient(NutrientAmount::Calcium(170.0)),
                    Component::Nutrient(NutrientAmount::Magnesium(48.0)),
                    Component::Nutrient(NutrientAmount::Sulfur(150.0)),
                    Component::Nutrient(NutrientAmount::Iron(2.0)),
                    Component::Nutrient(NutrientAmount::Manganese(0.55)),
                    Component::Nutrient(NutrientAmount::Copper(0.05)),
                    Component::Nutrient(NutrientAmount::Zinc(0.33)),
                    Component::Nutrient(NutrientAmount::Boron(0.28)),
                    Component::Nutrient(NutrientAmount::Molybdenum(0.05)),
                    Component::NitrogenForm(NitrogenForm::Nitrate(170.0)),
                    Component::NitrogenForm(NitrogenForm::Ammonium(19.0)),
                ],
            ),
            Profile::from(
                "bloom",
                vec![
                    Component::Nutrient(NutrientAmount::Nitrogen(140.0)),
                    Component::Nutrient(NutrientAmount::Phosphorus(39.0)),
                    Component::Nutrient(NutrientAmount::Potassium(400.0)),
                    Component::Nutrient(NutrientAmount::Calcium(190.0)),
                    Component::Nutrient(NutrientAmount::Magnesium(60.0)),
                    Component::Nutrient(NutrientAmount::Sulfur(150.0)),
                    Component::Nutrient(NutrientAmount::Iron(2.0)),
                    Component::Nutrient(NutrientAmount::Manganese(0.55)),
                    Component::Nutrient(NutrientAmount::Copper(0.05)),
                    Component::Nutrient(NutrientAmount::Zinc(0.33)),
                    Component::Nutrient(NutrientAmount::Boron(0.28)),
                    Component::Nutrient(NutrientAmount::Molybdenum(0.05)),
                    Component::NitrogenForm(NitrogenForm::Nitrate(170.0)),
                    Component::NitrogenForm(NitrogenForm::Ammonium(19.0)),
                ],
            ),
        ];

        for profile in profiles {
            self.add(profile);
        }
    }
}
