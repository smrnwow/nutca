use crate::model::chemistry::Nutrient;
use crate::model::profiles::{Profile, ProfilesListing};
use crate::storage::Error;
use rusqlite::{params, Connection};
use std::rc::Rc;

#[derive(Debug)]
pub struct Profiles {
    connection: Rc<Connection>,
}

impl Profiles {
    pub fn new(connection: Rc<Connection>) -> Result<Self, Box<dyn std::error::Error>> {
        let storage = Self { connection };

        storage.setup()?;

        storage.seed()?;

        Ok(storage)
    }

    pub fn add(&self, profile: Profile) -> Result<(), Box<dyn std::error::Error>> {
        let data = serde_json::to_string(&profile)?;

        self.connection.execute(
            "INSERT INTO profiles (id, data) VALUES (?1, ?2)",
            params![profile.id(), data],
        )?;

        Ok(())
    }

    pub fn get(&self, profile_id: String) -> Result<Profile, Box<dyn std::error::Error>> {
        let mut statement = self
            .connection
            .prepare("SELECT * FROM profiles WHERE id = ?1")?;

        let response = statement.query_map(params![profile_id], |row| {
            let data: String = row.get(1)?;
            Ok(data)
        })?;

        match response.last() {
            Some(profile) => Ok(serde_json::from_str(&profile?)?),
            None => Err(Box::new(Error::new("not found"))),
        }
    }

    pub fn update(&self, profile: Profile) -> Result<(), Box<dyn std::error::Error>> {
        let data = serde_json::to_string(&profile)?;

        self.connection
            .prepare("UPDATE profiles SET data = ?2 WHERE id = ?1")?
            .execute(params![profile.id(), data])?;

        Ok(())
    }

    pub fn delete(&self, profile_id: String) -> Result<(), Box<dyn std::error::Error>> {
        self.connection
            .prepare("DELETE FROM profiles WHERE id = ?1")?
            .execute(params![profile_id])?;

        Ok(())
    }

    pub fn list(&self) -> Result<ProfilesListing, Box<dyn std::error::Error>> {
        let mut statement = self.connection.prepare("SELECT * FROM profiles")?;

        let response = statement.query_map([], |row| {
            let data: String = row.get(1)?;
            Ok(data)
        })?;

        let mut profiles = vec![];

        for item in response {
            let profile: Profile = serde_json::from_str(&item?)?;

            profiles.push(profile);
        }

        Ok(ProfilesListing::new(profiles))
    }

    fn setup(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.connection.execute(
            "CREATE TABLE profiles (
                    id TEXT PRIMARY KEY,
                    data TEXT NOT NULL
                )",
            (),
        )?;

        Ok(())
    }

    fn seed(&self) -> Result<(), Box<dyn std::error::Error>> {
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
            self.add(profile)?;
        }

        Ok(())
    }
}
