use crate::model::concentrates::fillers::{AutoFiller, Filler, ManualFiller};
use crate::model::concentrates::Concentrate;
use uuid::Uuid;

pub struct StockSolutionBuilder {
    id: Option<String>,
    name: String,
    filler: Filler,
}

impl StockSolutionBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            name: String::new(),
            filler: Filler::Manual(ManualFiller::new()),
        }
    }

    pub fn with_id(&mut self, id: Option<String>) -> &mut Self {
        match id {
            Some(id) => self.id = Some(id),
            None => self.id = Some(Uuid::new_v4().to_string()),
        };

        self
    }

    pub fn with_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    pub fn with_auto_filler(&mut self, auto_filler: AutoFiller) -> &mut Self {
        self.filler = Filler::Auto(auto_filler);
        self
    }

    pub fn with_manual_filler(&mut self, manual_filler: ManualFiller) -> &mut Self {
        self.filler = Filler::Manual(manual_filler);
        self
    }

    pub fn build(&self) -> Concentrate {
        Concentrate {
            id: self.id.clone().unwrap_or(Uuid::new_v4().to_string()),
            name: self.name.clone(),
            filler: self.filler.clone(),
        }
    }
}
