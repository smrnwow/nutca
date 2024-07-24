use nutca::solutions::FertilizerWeight;

#[derive(Clone, Debug, PartialEq)]
pub struct SolutionFertilizers {
    fertilizers: Vec<FertilizerWeight>,
    page_index: usize,
    limit: usize,
}

impl SolutionFertilizers {
    pub fn new(fertilizers: Vec<FertilizerWeight>) -> Self {
        Self {
            fertilizers,
            page_index: 1,
            limit: 8,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.fertilizers.len() == 0
    }

    pub fn total(&self) -> usize {
        self.fertilizers.len()
    }

    pub fn page_index(&self) -> usize {
        self.page_index
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn paginate(&mut self, page_index: usize) {
        self.page_index = page_index;
    }

    pub fn items(&self) -> Vec<FertilizerWeight> {
        let start = (self.page_index - 1) * self.limit;

        let end = (self.page_index * self.limit) - 1;

        if end < self.fertilizers.len() {
            Vec::from(&self.fertilizers[start..=end])
        } else {
            Vec::from(&self.fertilizers[start..])
        }
    }
}
