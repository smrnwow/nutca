use crate::model::chemistry::NutrientAmount;
use crate::model::profiles::{Profile, ProfileBuilder};
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
            ProfileBuilder::new()
                .new_id()
                .name("UA CEAC Recipe")
                .nutrient_requirement(NutrientAmount::Nitrogen(189.0))
                .nutrient_requirement(NutrientAmount::NitrogenNitrate(189.0))
                .nutrient_requirement(NutrientAmount::NitrogenAmmonium(0.0))
                .nutrient_requirement(NutrientAmount::Phosphorus(39.0))
                .nutrient_requirement(NutrientAmount::Potassium(341.0))
                .nutrient_requirement(NutrientAmount::Calcium(170.0))
                .nutrient_requirement(NutrientAmount::Magnesium(48.0))
                .nutrient_requirement(NutrientAmount::Sulfur(150.0))
                .nutrient_requirement(NutrientAmount::Iron(2.0))
                .nutrient_requirement(NutrientAmount::Manganese(0.55))
                .nutrient_requirement(NutrientAmount::Copper(0.05))
                .nutrient_requirement(NutrientAmount::Zinc(0.33))
                .nutrient_requirement(NutrientAmount::Boron(0.28))
                .nutrient_requirement(NutrientAmount::Molybdenum(0.05))
                .build(),
            ProfileBuilder::new()
                .new_id()
                .name("Modified Sonneveld`s solution")
                .nutrient_requirement(NutrientAmount::Nitrogen(150.0))
                .nutrient_requirement(NutrientAmount::NitrogenNitrate(150.0))
                .nutrient_requirement(NutrientAmount::NitrogenAmmonium(0.0))
                .nutrient_requirement(NutrientAmount::Phosphorus(31.0))
                .nutrient_requirement(NutrientAmount::Potassium(210.0))
                .nutrient_requirement(NutrientAmount::Calcium(90.0))
                .nutrient_requirement(NutrientAmount::Magnesium(24.0))
                .nutrient_requirement(NutrientAmount::Sulfur(120.0))
                .nutrient_requirement(NutrientAmount::Iron(1.0))
                .nutrient_requirement(NutrientAmount::Manganese(0.25))
                .nutrient_requirement(NutrientAmount::Copper(0.023))
                .nutrient_requirement(NutrientAmount::Zinc(0.13))
                .nutrient_requirement(NutrientAmount::Boron(0.16))
                .nutrient_requirement(NutrientAmount::Molybdenum(0.024))
                .build(),
            ProfileBuilder::new()
                .new_id()
                .name("Hoagland solution")
                .nutrient_requirement(NutrientAmount::Nitrogen(210.0))
                .nutrient_requirement(NutrientAmount::NitrogenNitrate(210.0))
                .nutrient_requirement(NutrientAmount::NitrogenAmmonium(0.0))
                .nutrient_requirement(NutrientAmount::Phosphorus(31.0))
                .nutrient_requirement(NutrientAmount::Potassium(234.0))
                .nutrient_requirement(NutrientAmount::Calcium(200.0))
                .nutrient_requirement(NutrientAmount::Magnesium(50.0))
                .nutrient_requirement(NutrientAmount::Sulfur(64.0))
                .nutrient_requirement(NutrientAmount::Iron(3.0))
                .nutrient_requirement(NutrientAmount::Manganese(0.5))
                .nutrient_requirement(NutrientAmount::Copper(0.02))
                .nutrient_requirement(NutrientAmount::Zinc(0.05))
                .nutrient_requirement(NutrientAmount::Boron(0.5))
                .nutrient_requirement(NutrientAmount::Molybdenum(0.01))
                .build(),
            ProfileBuilder::new()
                .new_id()
                .name("University of Arizona Hydroponic Tomato Formula")
                .nutrient_requirement(NutrientAmount::Nitrogen(210.0))
                .nutrient_requirement(NutrientAmount::NitrogenNitrate(210.0))
                .nutrient_requirement(NutrientAmount::NitrogenAmmonium(0.0))
                .nutrient_requirement(NutrientAmount::Phosphorus(60.0))
                .nutrient_requirement(NutrientAmount::Potassium(350.0))
                .nutrient_requirement(NutrientAmount::Calcium(180.0))
                .nutrient_requirement(NutrientAmount::Magnesium(50.0))
                .nutrient_requirement(NutrientAmount::Sulfur(70.0))
                .nutrient_requirement(NutrientAmount::Iron(2.0))
                .nutrient_requirement(NutrientAmount::Manganese(0.5))
                .nutrient_requirement(NutrientAmount::Copper(0.05))
                .nutrient_requirement(NutrientAmount::Zinc(0.1))
                .nutrient_requirement(NutrientAmount::Boron(0.5))
                .nutrient_requirement(NutrientAmount::Molybdenum(0.01))
                .build(),
        ];

        for profile in profiles {
            self.add(profile)?;
        }

        Ok(())
    }
}
