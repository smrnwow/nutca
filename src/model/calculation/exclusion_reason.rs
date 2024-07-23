#[derive(Copy, Clone, Debug)]
pub enum ExclusionReason {
    DuplicatedNutrientSource,
    NullishNutrientRequirement,
    NegativeAmount,
}
