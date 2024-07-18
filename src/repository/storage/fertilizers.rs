use crate::model::fertilizers::labels::{Component, Units};
use crate::model::fertilizers::{Fertilizer, FertilizerBuilder};
use crate::repository::{Error, RepositoryError};
use rusqlite::{params, Connection};
use std::rc::Rc;

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
            FertilizerBuilder::new()
                .name("Кальциевая селитра (3-х водная)")
                .formula("Ca(NO3)2*3H20")
                .build(),
            FertilizerBuilder::new()
                .name("Кальциевая селитра (4-х водная)")
                .formula("Ca(NO3)2*4H20")
                .build(),
            FertilizerBuilder::new()
                .name("Калиевая селитра")
                .formula("KNO3")
                .build(),
            FertilizerBuilder::new()
                .name("Аммиачная селитра")
                .formula("NH4NO3")
                .build(),
            FertilizerBuilder::new()
                .name("Сульфат магния")
                .formula("MgSO4*7H2O")
                .build(),
            FertilizerBuilder::new()
                .name("Сульфат калия")
                .formula("K2SO4")
                .build(),
            FertilizerBuilder::new()
                .name("Монофосфат калия")
                .formula("KH2PO4")
                .build(),
            FertilizerBuilder::new()
                .name("Кристалон цветочный")
                .vendor("fertika")
                .label_units(Units::Percent)
                .label_component(Component::Nitrogen(19.))
                .label_component(Component::PhosphorPentoxide(6.))
                .label_component(Component::PotassiumOxide(20.))
                .label_component(Component::MagnesiumOxide(3.))
                .label_component(Component::Sulfur(3.))
                .label_component(Component::SulfurTrioxide(7.5))
                .label_component(Component::Iron(0.07))
                .label_component(Component::Manganese(0.04))
                .label_component(Component::Copper(0.01))
                .label_component(Component::Zinc(0.025))
                .label_component(Component::Boron(0.025))
                .label_component(Component::Molybdenum(0.004))
                .build(),
            FertilizerBuilder::new()
                .name("Унифлор микро")
                .label_units(Units::WeightVolume)
                .label_component(Component::Magnesium(15000.))
                .label_component(Component::Iron(3200.))
                .label_component(Component::Manganese(1600.))
                .label_component(Component::Boron(1200.))
                .label_component(Component::Zinc(360.))
                .label_component(Component::Copper(320.))
                .label_component(Component::Molybdenum(102.))
                .liquid(true)
                .build(),
            FertilizerBuilder::new()
                .name("Хелат железа DTPA")
                .label_units(Units::Percent)
                .label_component(Component::Iron(10.))
                .build(),
            FertilizerBuilder::new()
                .name("Сульфат марганца")
                .formula("MnSO4*H2O")
                .build(),
            FertilizerBuilder::new()
                .name("Борная кислота")
                .formula("H3BO3")
                .build(),
            FertilizerBuilder::new()
                .name("Молибденовая кислота")
                .formula("Na2MoO4*2H2O")
                .build(),
            FertilizerBuilder::new()
                .name("Сульфат цинка")
                .formula("ZnSO4*7H2O")
                .build(),
            FertilizerBuilder::new()
                .name("Сульфат меди")
                .formula("CuSO4*5H2O")
                .build(),
        ];

        for fertilizer in fertilizers {
            self.add(fertilizer)?;
        }

        for index in 0..=100 {
            let fertilizer = FertilizerBuilder::new()
                .name(format!("Монофосфат калия {}", index))
                .formula("KH2PO4")
                .build();

            self.add(fertilizer)?;
        }

        Ok(())
    }
}
