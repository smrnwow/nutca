use crate::model::fertilizers::FertilizerAmount;

#[derive(Clone, Debug, PartialEq)]
pub struct FertilizersUsed {
    fertilizers: Vec<FertilizerAmount>,
    limit: usize,
    page_index: usize,
}

impl FertilizersUsed {
    pub fn new(fertilizers: Vec<&FertilizerAmount>) -> Self {
        Self {
            fertilizers: fertilizers.into_iter().cloned().collect(),
            limit: 8,
            page_index: 1,
        }
    }

    pub fn set_fertilizers(&mut self, fertilizers: Vec<&FertilizerAmount>) {
        self.fertilizers = fertilizers.into_iter().cloned().collect();
    }

    pub fn page_index(&self) -> usize {
        self.page_index
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn count(&self) -> usize {
        self.fertilizers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    pub fn paginate(&mut self, page_index: usize) {
        self.page_index = page_index;
    }

    pub fn items(&self) -> Vec<FertilizerAmount> {
        let start = (self.page_index - 1) * self.limit;

        let end = (self.page_index * self.limit) - 1;

        if end < self.fertilizers.len() {
            return self.fertilizers[start..=end].iter().cloned().collect();
        } else {
            return self.fertilizers[start..].iter().cloned().collect();
        }
    }
}
