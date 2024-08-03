use super::errors::Error;

pub fn write_json<T: Serialize>(item: T, out_path: &str) -> Result<(), Error> {
    let serialized = serde_json::to_string(&item);
    let out_file = File::create(out_path);
    out_file.write_all(&serialized);
    Ok(())
}

pub fn write_vec<T: Serialize>(items: Vec<T>, out_path_base: &str) -> Result<(), Error> {}
