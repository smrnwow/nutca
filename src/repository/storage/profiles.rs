use crate::model::chemistry::{NutrientAmount, Nutrients};
use crate::model::profiles::Profile;
use crate::repository::{Error, RepositoryError};
use rusqlite::{params, Connection};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Profiles {
    connection: Rc<Connection>,
}

impl Profiles {
    pub fn new(connection: Rc<Connection>) -> Result<Self, Error> {
        let storage = Self { connection };

        storage.setup()?;

        storage.seed()?;

        Ok(storage)
    }

    pub fn add(&self, profile: Profile) -> Result<(), Error> {
        let data = serde_json::to_string(&profile)?;

        self.connection.execute(
            "INSERT INTO profiles (id, data) VALUES (?1, ?2)",
            params![profile.id(), data],
        )?;

        Ok(())
    }

    pub fn get(&self, profile_id: String) -> Result<Profile, Error> {
        let mut statement = self
            .connection
            .prepare("SELECT * FROM profiles WHERE id = ?1")?;

        let response = statement.query_map(params![profile_id], |row| {
            let data: String = row.get(1)?;
            Ok(data)
        })?;

        match response.last() {
            Some(profile) => Ok(serde_json::from_str(&profile?)?),
            None => Err(Box::new(RepositoryError::new("not found"))),
        }
    }

    pub fn update(&self, profile: Profile) -> Result<(), Error> {
        let data = serde_json::to_string(&profile)?;

        self.connection
            .prepare("UPDATE profiles SET data = ?2 WHERE id = ?1")?
            .execute(params![profile.id(), data])?;

        Ok(())
    }

    pub fn delete(&self, profile_id: String) -> Result<(), Error> {
        self.connection
            .prepare("DELETE FROM profiles WHERE id = ?1")?
            .execute(params![profile_id])?;

        Ok(())
    }

    pub fn list(&self) -> Result<Vec<Profile>, Error> {
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

        Ok(profiles)
    }

    fn setup(&self) -> Result<(), Error> {
        self.connection.execute(
            "CREATE TABLE profiles (
                    id TEXT PRIMARY KEY,
                    data TEXT NOT NULL
                )",
            (),
        )?;

        Ok(())
    }

    fn seed(&self) -> Result<(), Error> {
        let profiles = vec![
            Profile::from(
                "UA CEAC Recipe",
                Nutrients::from(vec![
                    NutrientAmount::Nitrogen(189.0),
                    NutrientAmount::NitrogenNitrate(189.0),
                    NutrientAmount::NitrogenAmmonium(0.0),
                    NutrientAmount::Phosphorus(39.0),
                    NutrientAmount::Potassium(341.0),
                    NutrientAmount::Calcium(170.0),
                    NutrientAmount::Magnesium(48.0),
                    NutrientAmount::Sulfur(150.0),
                    NutrientAmount::Iron(2.0),
                    NutrientAmount::Manganese(0.55),
                    NutrientAmount::Copper(0.05),
                    NutrientAmount::Zinc(0.33),
                    NutrientAmount::Boron(0.28),
                    NutrientAmount::Molybdenum(0.05),
                ]),
            ),
            Profile::from(
                "Modified Sonneveld`s solution",
                Nutrients::from(vec![
                    NutrientAmount::Nitrogen(150.0),
                    NutrientAmount::NitrogenNitrate(150.0),
                    NutrientAmount::NitrogenAmmonium(0.0),
                    NutrientAmount::Phosphorus(31.0),
                    NutrientAmount::Potassium(210.0),
                    NutrientAmount::Calcium(90.0),
                    NutrientAmount::Magnesium(24.0),
                    NutrientAmount::Sulfur(120.0),
                    NutrientAmount::Iron(1.0),
                    NutrientAmount::Manganese(0.25),
                    NutrientAmount::Copper(0.023),
                    NutrientAmount::Zinc(0.13),
                    NutrientAmount::Boron(0.16),
                    NutrientAmount::Molybdenum(0.024),
                ]),
            ),
            Profile::from(
                "Hoagland solution",
                Nutrients::from(vec![
                    NutrientAmount::Nitrogen(210.0),
                    NutrientAmount::NitrogenNitrate(210.0),
                    NutrientAmount::NitrogenAmmonium(0.0),
                    NutrientAmount::Phosphorus(31.0),
                    NutrientAmount::Potassium(234.0),
                    NutrientAmount::Calcium(200.0),
                    NutrientAmount::Magnesium(50.0),
                    NutrientAmount::Sulfur(64.0),
                    NutrientAmount::Iron(3.0),
                    NutrientAmount::Manganese(0.5),
                    NutrientAmount::Copper(0.02),
                    NutrientAmount::Zinc(0.05),
                    NutrientAmount::Boron(0.5),
                    NutrientAmount::Molybdenum(0.01),
                ]),
            ),
            Profile::from(
                "University of Arizona Hydroponic Tomato Formula",
                Nutrients::from(vec![
                    NutrientAmount::Nitrogen(210.0),
                    NutrientAmount::NitrogenNitrate(210.0),
                    NutrientAmount::NitrogenAmmonium(0.0),
                    NutrientAmount::Phosphorus(60.0),
                    NutrientAmount::Potassium(350.0),
                    NutrientAmount::Calcium(180.0),
                    NutrientAmount::Magnesium(50.0),
                    NutrientAmount::Sulfur(70.0),
                    NutrientAmount::Iron(2.0),
                    NutrientAmount::Manganese(0.5),
                    NutrientAmount::Copper(0.05),
                    NutrientAmount::Zinc(0.1),
                    NutrientAmount::Boron(0.5),
                    NutrientAmount::Molybdenum(0.01),
                ]),
            ),
        ];

        for profile in profiles {
            self.add(profile)?;
        }

        Ok(())
    }
}
