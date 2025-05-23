use plants::{graveyard::GraveyardPlant, growth_item::GrowthItem, log_item::LogItem};
use sqlite::Connection;
use std::{collections::HashMap, path::PathBuf};

pub mod db_man;
pub mod errors;
use errors::Error;

pub struct SQLiteDB {
    pub db_path: PathBuf,
    pub connection: Connection,
    pub date_format: String,
    pub plants_dir: PathBuf,
}

impl SQLiteDB {
    pub fn new(path: PathBuf) -> Result<SQLiteDB, Error> {
        let con = sqlite::open(path.clone())?;
        Ok(SQLiteDB {
            db_path: path,
            connection: con,
            date_format: "%d.%m.%Y".to_owned(),
            plants_dir: PathBuf::from("data").join("Plants"),
        })
    }

    pub fn read_rows(
        &mut self,
        query: &str,
        column_keys: Vec<&str>,
    ) -> Result<Vec<HashMap<String, String>>, Error> {
        let mut maps = vec![];
        let callback = |cols: &[(&str, Option<&str>)]| {
            let mut map = HashMap::new();
            for (key, val) in cols.into_iter() {
                let value = if let Some(val) = val { val } else { "" };
                if column_keys.contains(key) {
                    map.insert(format!("{}", key), format!("{}", value));
                }
            }
            maps.push(map);
            true
        };
        self.connection.iterate(query, callback)?;
        Ok(maps)
    }

    pub fn get_growth_plant(
        &mut self,
        plant_name: &str,
    ) -> Result<Vec<GrowthItem>, Box<dyn std::error::Error>> {
        let growth_query = format!("SELECT * FROM growth WHERE plant='{}'", plant_name);
        let growth_maps = self.read_rows(
            &growth_query,
            vec!["plant", "date", "height_cm", "width_cm", "health", "note"],
        )?;
        let mut growth = vec![];
        for mut map in growth_maps.into_iter() {
            map.insert("date_format".to_owned(), self.date_format.clone());
            let item: GrowthItem = map.try_into()?;
            growth.push(item)
        }
        Ok(growth)
    }

    pub fn get_logs_plant(
        &mut self,
        plant_name: &str,
    ) -> Result<Vec<LogItem>, Box<dyn std::error::Error>> {
        let log_query = format!("SELECT * FROM activities WHERE plant='{}'", plant_name);
        let log_maps = self.read_rows(&log_query, vec!["name", "date", "plant", "note"])?;

        let mut logs = vec![];
        for mut map in log_maps.into_iter() {
            map.insert("date_format".to_owned(), self.date_format.clone());
            let item: LogItem = map.try_into()?;
            logs.push(item);
        }
        Ok(logs)
    }

    pub fn add_to_graveyard(
        &mut self,
        plant: GraveyardPlant,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let fmt_plant = |plant: &GraveyardPlant| {
            format!(
                "('{}','{}','{}','{}','{}')",
                self.sanitize(&plant.name),
                self.sanitize(&plant.species),
                plant.planted.format(&self.date_format),
                plant.died.format(&self.date_format),
                self.sanitize(&plant.reason)
            )
        };
        let fields = "(name,species,planted,died,reason)";
        let mut graveyard_query = "INSERT INTO graveyard ".to_owned();
        graveyard_query += fields;
        graveyard_query += " VALUES ";
        graveyard_query += &fmt_plant(&plant);
        graveyard_query += ";";
        self.connection.execute(graveyard_query)?;
        Ok(())
    }

    pub fn sanitize<T: ToString>(&self, input: &T) -> String {
        input.to_string().replace('\'', "''")
    }
}
