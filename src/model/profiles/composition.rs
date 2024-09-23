use super::Stage;
use crate::model::chemistry::Nutrients;

pub enum Composition {
    Nutrients(Nutrients),
    Stages(Vec<Stage>),
}
