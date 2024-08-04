use std::path;

pub struct FileDB {
    pub plants_dir: String,
    pub species_dir: String,
    logs_dir: String,
    graveyard_csv: String,
    growth_csv: String,
    activities_csv: String,
    pub date_format: String,
    //REMOVE later
    pub plants_out_dir: String,
    pub species_out_dir: String,
    pub graveyard_out: String,
    pub activities_out: String,
    pub growth_out: String,
}

impl FileDB {
    pub fn get_activities_filepath(&self) -> path::PathBuf {
        path::Path::new(&self.logs_dir).join(&self.activities_csv)
    }

    pub fn get_graveyard_filepath(&self) -> path::PathBuf {
        path::Path::new(&self.logs_dir).join(&self.graveyard_csv)
    }

    pub fn get_growth_filepath(&self) -> path::PathBuf {
        path::Path::new(&self.logs_dir).join(&self.growth_csv)
    }
}

pub fn get_default() -> FileDB {
    FileDB {
        plants_dir: "data/Plants".to_owned(),
        species_dir: "data/Species".to_owned(),
        logs_dir: "data/Logs".to_owned(),
        graveyard_csv: "Graveyard.csv".to_owned(),
        growth_csv: "Growth.csv".to_owned(),
        activities_csv: "Activities.csv".to_owned(),
        date_format: "%d.%m.%Y".to_owned(),
        plants_out_dir: "data_new/Plants".to_owned(),
        species_out_dir: "data_new/Species".to_owned(),
        graveyard_out: "data_new/Logs/Graveyard.csv".to_owned(),
        activities_out: "data_new/Logs/Activities.csv".to_owned(),
        growth_out: "data_new/Logs/Growth.csv".to_owned(),
    }
}
