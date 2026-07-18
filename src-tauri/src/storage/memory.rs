use crate::domain::{Category, ChronicleObject, Entry};

use super::ChronologyRepository;

#[derive(Default)]
pub struct MemoryChronologyRepository {
    categories: Vec<Category>,
    objects: Vec<ChronicleObject>,
    entries: Vec<Entry>,
}

impl ChronologyRepository for MemoryChronologyRepository {
    fn save_category(&mut self, category: Category) {
        self.categories.push(category);
    }

    fn save_object(&mut self, object: ChronicleObject) {
        self.objects.push(object);
    }

    fn save_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    fn categories(&self) -> Vec<Category> {
        self.categories.clone()
    }

    fn objects(&self) -> Vec<ChronicleObject> {
        self.objects.clone()
    }

    fn entries(&self) -> Vec<Entry> {
        self.entries.clone()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn stores_category() {
        let mut repository = MemoryChronologyRepository::default();

        let category = Category::new("Garden").unwrap();

        repository.save_category(category);

        assert_eq!(repository.categories().len(), 1);
    }
}
