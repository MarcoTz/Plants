use database::database_manager::DatabaseManager;
pub struct Renderer<T: DatabaseManager> {
    database_manager: T,
}
