use super::{
    errors::{BadValue, Error},
    parse_json::{get_i64, get_map, get_option_i64, get_str},
};
use serde_json::Value;

pub struct Photo {
    sizes: Vec<PhotoSize>,
}
pub struct PhotoSize {
    file_id: String,
    file_unique_id: String,
    width: i64,
    height: i64,
    file_size: Option<i64>,
}

impl TryFrom<Value> for PhotoSize {
    type Error = Error;
    fn try_from(val: Value) -> Result<PhotoSize, Self::Error> {
        let mut val_map = get_map(val)?;

        let file_id = get_str(&mut val_map, "file_id")?;
        let file_unique_id = get_str(&mut val_map, "file_unique_id")?;
        let width = get_i64(&mut val_map, "width")?;
        let height = get_i64(&mut val_map, "height")?;
        let file_size = get_option_i64(&mut val_map, "file_size")?;

        Ok(PhotoSize {
            file_id,
            file_unique_id,
            width,
            height,
            file_size,
        })
    }
}

impl TryFrom<Value> for Photo {
    type Error = Error;
    fn try_from(val: Value) -> Result<Photo, Self::Error> {
        if let Value::Array(vals) = val {
            let mut sizes = vec![];

            for size_val in vals {
                let new_size = size_val.try_into()?;
                sizes.push(new_size);
            }

            Ok(Photo { sizes })
        } else {
            Err(BadValue {
                field: "Entire Value".to_owned(),
                val,
            }
            .into())
        }
    }
}
