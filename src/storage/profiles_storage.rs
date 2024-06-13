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
                "grow",
                vec![
                    Nutrient::Nitrogen(189.0),
                    Nutrient::NitrogenNitrate(170.0),
                    Nutrient::NitrogenAmmonium(19.0),
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
                "bloom",
                vec![
                    Nutrient::Nitrogen(140.0),
                    Nutrient::NitrogenNitrate(121.0),
                    Nutrient::NitrogenAmmonium(19.0),
                    Nutrient::Phosphorus(39.0),
                    Nutrient::Potassium(400.0),
                    Nutrient::Calcium(190.0),
                    Nutrient::Magnesium(60.0),
                    Nutrient::Sulfur(150.0),
                    Nutrient::Iron(2.0),
                    Nutrient::Manganese(0.55),
                    Nutrient::Copper(0.05),
                    Nutrient::Zinc(0.33),
                    Nutrient::Boron(0.28),
                    Nutrient::Molybdenum(0.05),
                ],
            ),
        ];

        for profile in profiles {
            self.add(profile);
        }
    }
}
