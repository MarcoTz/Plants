use database::json_db::load_plants;

fn main() {
    let plant_list = load_plants();
    match plant_list {
        Ok(res) => println!("{:?}", res),
        Err(err) => panic!("Could not load file, {:?}", err),
    }
}
