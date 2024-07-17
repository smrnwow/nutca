use crate::model::fertilizers::Fertilizer;
use crate::model::formulas::Formula;
use crate::model::labels::{Component, Label, Units};
use crate::repository::{Error, RepositoryError};
use rusqlite::{params, Connection};
use std::rc::Rc;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Fertilizers {
    connection: Rc<Connection>,
}

impl Fertilizers {
    pub fn new(connection: Rc<Connection>) -> Result<Self, Error> {
        let storage = Self { connection };

        storage.setup()?;

        storage.seed()?;

        Ok(storage)
    }

    pub fn add(&self, fertilizer: Fertilizer) -> Result<(), Error> {
        let data = serde_json::to_string(&fertilizer)?;

        self.connection.execute(
            "INSERT INTO fertilizers (id, data) VALUES (?1, ?2)",
            params![fertilizer.id(), data],
        )?;

        Ok(())
    }

    pub fn get(&self, fertilizer_id: String) -> Result<Fertilizer, Error> {
        let mut statement = self
            .connection
            .prepare("SELECT * FROM fertilizers WHERE id = ?1")?;

        let response = statement.query_map(params![fertilizer_id], |row| {
            let data: String = row.get(1)?;
            Ok(data)
        })?;

        match response.last() {
            Some(fertilizer) => Ok(serde_json::from_str(&fertilizer?)?),
            None => Err(Box::new(RepositoryError::new("not found"))),
        }
    }

    pub fn update(&self, fertilizer: Fertilizer) -> Result<(), Error> {
        let data = serde_json::to_string(&fertilizer)?;

        self.connection
            .prepare("UPDATE fertilizers SET data = ?2 WHERE id = ?1")?
            .execute(params![fertilizer.id(), data])?;

        Ok(())
    }

    pub fn delete(&self, fertilizer_id: String) -> Result<(), Error> {
        self.connection
            .prepare("DELETE FROM fertilizers WHERE id = ?1")?
            .execute(params![fertilizer_id])?;

        Ok(())
    }

    pub fn list(&self) -> Result<Vec<Fertilizer>, Error> {
        let mut statement = self.connection.prepare("SELECT * FROM fertilizers")?;

        let response = statement.query_map([], |row| {
            let data: String = row.get(1)?;
            Ok(data)
        })?;

        let mut fertilizers = vec![];

        for item in response {
            let fertilizer = serde_json::from_str::<Fertilizer>(&item?)?;
            fertilizers.push(fertilizer);
        }

        Ok(fertilizers)
    }

    fn setup(&self) -> Result<(), Error> {
        self.connection.execute(
            "CREATE TABLE fertilizers (
                    id TEXT PRIMARY KEY,
                    data TEXT NOT NULL
                )",
            (),
        )?;

        Ok(())
    }

    fn seed(&self) -> Result<(), Error> {
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
                ))
                .with_liquid(true),
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
            self.add(fertilizer)?;
        }

        for index in 0..=100 {
            let fertilizer = Fertilizer::build()
                .with_id(Uuid::new_v4().to_string())
                .with_name(format!("Монофосфат калия {}", index))
                .with_formula(Formula::from("KH2PO4"));

            self.add(fertilizer)?;
        }

        Ok(())
    }
}
