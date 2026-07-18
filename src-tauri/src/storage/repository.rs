use crate::domain::{Category, ChronicleObject, Entry};

pub trait ChronologyRepository {
    fn save_category(&mut self, category: Category);

    fn save_object(&mut self, object: ChronicleObject);

    fn save_entry(&mut self, entry: Entry);

    fn categories(&self) -> Vec<Category>;

    fn objects(&self) -> Vec<ChronicleObject>;

    fn entries(&self) -> Vec<Entry>;
}
