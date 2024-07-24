#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ExclusionReason {
    DuplicatedNutrientSource,
    NullishNutrientRequirement,
    NegativeAmount,
    RedurantFertilizer,
}
