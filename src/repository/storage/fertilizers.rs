use crate::model::fertilizers::{Fertilizer, FertilizerBuilder, LabelComponent, LabelUnits};
use crate::repository::{Error, RepositoryError};
use rusqlite::{named_params, params, Connection};
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
            "INSERT INTO fertilizers (id, name, data) VALUES (?1, ?2, ?3)",
            params![fertilizer.id(), fertilizer.name().to_lowercase(), data],
        )?;

        Ok(())
    }

    pub fn get(&self, fertilizer_id: String) -> Result<Fertilizer, Error> {
        let mut statement = self
            .connection
            .prepare("SELECT * FROM fertilizers WHERE id = ?1")?;

        let response = statement.query_map(params![fertilizer_id], |row| {
            let data: String = row.get(2)?;
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

    pub fn search(
        &self,
        query: String,
        not_in: &[String],
        limit: usize,
        offset: usize,
    ) -> Result<Vec<Fertilizer>, Error> {
        let not_in_items = not_in
            .iter()
            .map(|id| format!("\"{}\"", id))
            .collect::<Vec<String>>()
            .join(",");

        let query_str = format!("SELECT * FROM fertilizers WHERE name LIKE '%' || :search || '%' AND id NOT IN ({}) LIMIT :limit OFFSET :offset", not_in_items);

        let mut statement = self.connection.prepare(&query_str)?;

        let response = statement.query_map(
            named_params! {
                ":search": query,
                ":limit": limit,
                ":offset": offset,
            },
            |row| {
                let data: String = row.get(2)?;
                Ok(data)
            },
        )?;

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
                name TEXT NOT NULL,
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
                .formula("Ca(NO3)2*3H2O")
                .build(),
            FertilizerBuilder::new()
                .name("Кальциевая селитра (4-х водная)")
                .formula("Ca(NO3)2*4H2O")
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
                .label_units(LabelUnits::Percent)
                .label_component(LabelComponent::Nitrogen(19.))
                .label_component(LabelComponent::PhosphorusPentoxide(6.))
                .label_component(LabelComponent::PotassiumOxide(20.))
                .label_component(LabelComponent::MagnesiumOxide(3.))
                .label_component(LabelComponent::Sulfur(3.))
                .label_component(LabelComponent::SulfurTrioxide(7.5))
                .label_component(LabelComponent::Iron(0.07))
                .label_component(LabelComponent::Manganese(0.04))
                .label_component(LabelComponent::Copper(0.01))
                .label_component(LabelComponent::Zinc(0.025))
                .label_component(LabelComponent::Boron(0.025))
                .label_component(LabelComponent::Molybdenum(0.004))
                .build(),
            FertilizerBuilder::new()
                .name("Унифлор микро")
                .label_units(LabelUnits::WeightVolume)
                .label_component(LabelComponent::Magnesium(15000.))
                .label_component(LabelComponent::Iron(3200.))
                .label_component(LabelComponent::Manganese(1600.))
                .label_component(LabelComponent::Boron(1200.))
                .label_component(LabelComponent::Zinc(360.))
                .label_component(LabelComponent::Copper(320.))
                .label_component(LabelComponent::Molybdenum(102.))
                .liquid(true)
                .build(),
            FertilizerBuilder::new()
                .name("Хелат железа DTPA")
                .label_units(LabelUnits::Percent)
                .label_component(LabelComponent::Iron(10.))
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

        Ok(())
    }
}
