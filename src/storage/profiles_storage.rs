use super::provider::Provider;
use crate::model::chemistry::Nutrient;
use crate::model::profiles::{Profile, ProfilesListing};
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
                    id TEXT PRIMARY KEY,
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
            .execute(
                "INSERT INTO profiles (id, data) VALUES (?1, ?2)",
                params![profile.id(), data],
            )
            .unwrap();

        self.storage.connection().last_insert_rowid()
    }

    pub fn get(&self, profile_id: String) -> Option<Profile> {
        let query = format!("SELECT * FROM profiles WHERE id = \"{profile_id}\"");

        let response = self.storage.connection().prepare(query.as_str());

        match response {
            Ok(mut result) => {
                let profiles: Vec<Profile> = result
                    .query_map([], |row| {
                        let data: String = row.get(1).unwrap();

                        Ok(serde_json::from_str::<Profile>(&data).expect("Failed to deserialize"))
                    })
                    .unwrap()
                    .map(|profile| profile.unwrap())
                    .collect();

                if profiles.len() > 0 {
                    return Some(profiles.get(0).unwrap().clone());
                }

                None
            }
            Err(error) => {
                println!("profile get error {:#?}", error);

                None
            }
        }
    }

    pub fn update(&self, profile: Profile) {
        let data = serde_json::to_string(&profile).expect("Failed to serialize");

        let query = "UPDATE profiles SET data = ?2 WHERE id = ?1";

        let mut statement = self.storage.connection().prepare(query).unwrap();

        statement.execute(params![profile.id(), data]).unwrap();
    }

    pub fn delete(&self, profile_id: String) {
        let query = "DELETE FROM profiles WHERE id = ?1";

        let mut statement = self.storage.connection().prepare(query).unwrap();

        statement.execute(params![profile_id]).unwrap();
    }

    pub fn list(&self) -> ProfilesListing {
        let statement = self.storage.connection().prepare("SELECT * FROM profiles");

        match statement {
            Ok(mut query) => {
                let profiles = query
                    .query_map([], |row| {
                        let data: String = row.get(1).unwrap();

                        Ok(serde_json::from_str::<Profile>(&data).expect("Failed to deserialize"))
                    })
                    .unwrap()
                    .map(|profile| profile.unwrap())
                    .collect();

                ProfilesListing::new(profiles)
            }

            Err(_) => ProfilesListing::new(vec![]),
        }
    }

    fn seed(&self) {
        let profiles = vec![
            Profile::from(
                "UA CEAC Recipe",
                vec![
                    Nutrient::Nitrogen(189.0),
                    Nutrient::NitrogenNitrate(189.0),
                    Nutrient::NitrogenAmmonium(0.0),
                    Nutrient::Phosphorus(39.0),
                    Nutrient::Potassium(341.0),
                    Nutrient::Calcium(170.0),
                    Nutrient::Magnesium(48.0),
                    Nutrient::Sulfur(150.0),
                    Nutrient::Iron(2.0),
                    Nutrient::Manganese(0.55),
                    Nutrient::Copper(0.05),
                    Nutrient::Zinc(0.33),
                    Nutrient::Boron(0.28),
                    Nutrient::Molybdenum(0.05),
                ],
            ),
            Profile::from(
                "Modified Sonneveld`s solution",
                vec![
                    Nutrient::Nitrogen(150.0),
                    Nutrient::NitrogenNitrate(150.0),
                    Nutrient::NitrogenAmmonium(0.0),
                    Nutrient::Phosphorus(31.0),
                    Nutrient::Potassium(210.0),
                    Nutrient::Calcium(90.0),
                    Nutrient::Magnesium(24.0),
                    Nutrient::Sulfur(120.0),
                    Nutrient::Iron(1.0),
                    Nutrient::Manganese(0.25),
                    Nutrient::Copper(0.023),
                    Nutrient::Zinc(0.13),
                    Nutrient::Boron(0.16),
                    Nutrient::Molybdenum(0.024),
                ],
            ),
            Profile::from(
                "Hoagland solution",
                vec![
                    Nutrient::Nitrogen(210.0),
                    Nutrient::NitrogenNitrate(210.0),
                    Nutrient::NitrogenAmmonium(0.0),
                    Nutrient::Phosphorus(31.0),
                    Nutrient::Potassium(234.0),
                    Nutrient::Calcium(200.0),
                    Nutrient::Magnesium(50.0),
                    Nutrient::Sulfur(64.0),
                    Nutrient::Iron(3.0),
                    Nutrient::Manganese(0.5),
                    Nutrient::Copper(0.02),
                    Nutrient::Zinc(0.05),
                    Nutrient::Boron(0.5),
                    Nutrient::Molybdenum(0.01),
                ],
            ),
            Profile::from(
                "University of Arizona Hydroponic Tomato Formula",
                vec![
                    Nutrient::Nitrogen(210.0),
                    Nutrient::NitrogenNitrate(210.0),
                    Nutrient::NitrogenAmmonium(0.0),
                    Nutrient::Phosphorus(60.0),
                    Nutrient::Potassium(350.0),
                    Nutrient::Calcium(180.0),
                    Nutrient::Magnesium(50.0),
                    Nutrient::Sulfur(70.0),
                    Nutrient::Iron(2.0),
                    Nutrient::Manganese(0.5),
                    Nutrient::Copper(0.05),
                    Nutrient::Zinc(0.1),
                    Nutrient::Boron(0.5),
                    Nutrient::Molybdenum(0.01),
                ],
            ),
        ];

        for profile in profiles {
            self.add(profile);
        }
    }
}
