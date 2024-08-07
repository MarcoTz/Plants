use plants::plant::Plant;

pub trait DatabaseManager {
    fn get_all_plants(&self) -> Vec<Plant>;
}
