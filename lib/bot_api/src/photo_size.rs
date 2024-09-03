use super::{
    errors::{BadValue, Error},
    parse_json::{get_i64, get_map, get_option_i64, get_str},
};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Photo {
    pub sizes: Vec<PhotoSize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhotoSize {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i64,
    pub height: i64,
    pub file_size: Option<i64>,
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

#[cfg(test)]
mod photo_tests {

    use super::{Photo, PhotoSize};
    use serde_json::{Map, Number, Value};

    fn example_photo_size_val() -> Map<String, Value> {
        let mut val_map = Map::new();
        val_map.insert("file_id".to_owned(), Value::String("id".to_owned()));
        val_map.insert(
            "file_unique_id".to_owned(),
            Value::String("unique_id".to_owned()),
        );
        val_map.insert("width".to_owned(), Value::Number(Number::from(1)));
        val_map.insert("height".to_owned(), Value::Number(Number::from(1)));
        val_map.insert("file_size".to_owned(), Value::Number(Number::from(1)));
        val_map
    }

    fn example_photo_size_parsed() -> PhotoSize {
        PhotoSize {
            file_id: "id".to_owned(),
            file_unique_id: "unique_id".to_owned(),
            width: 1,
            height: 1,
            file_size: Some(1),
        }
    }

    #[test]
    fn size_from_val() {
        let result =
            <Value as TryInto<PhotoSize>>::try_into(Value::Object(example_photo_size_val()))
                .unwrap();
        let expected = example_photo_size_parsed();
        assert_eq!(result, expected)
    }

    #[test]
    fn size_from_val_no_size() {
        let mut val_map = example_photo_size_val();
        val_map.remove("file_size");
        let mut expected = example_photo_size_parsed();
        expected.file_size = None;
        let result = <Value as TryInto<PhotoSize>>::try_into(Value::Object(val_map)).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn size_from_val_no_id() {
        let mut val_map = example_photo_size_val();
        val_map.remove("file_id");
        let result = <Value as TryInto<PhotoSize>>::try_into(Value::Object(val_map));
        assert!(result.is_err())
    }

    #[test]
    fn size_from_val_no_unique_id() {
        let mut val_map = example_photo_size_val();
        val_map.remove("file_unique_id");
        let result = <Value as TryInto<PhotoSize>>::try_into(Value::Object(val_map));
        assert!(result.is_err())
    }

    #[test]
    fn size_from_val_no_width() {
        let mut val_map = example_photo_size_val();
        val_map.remove("width");
        let result = <Value as TryInto<PhotoSize>>::try_into(Value::Object(val_map));
        assert!(result.is_err())
    }

    #[test]
    fn size_from_val_no_height() {
        let mut val_map = example_photo_size_val();
        val_map.remove("height");
        let result = <Value as TryInto<PhotoSize>>::try_into(Value::Object(val_map));
        assert!(result.is_err())
    }

    #[test]
    fn size_from_val_bad_filesize() {
        let mut val_map = example_photo_size_val();
        val_map.insert("file_size".to_owned(), Value::Bool(false));
        let result = <Value as TryInto<PhotoSize>>::try_into(Value::Object(val_map));
        assert!(result.is_err())
    }

    #[test]
    fn size_from_val_no_map() {
        let result = <Value as TryInto<PhotoSize>>::try_into(Value::String("bad value".to_owned()));
        assert!(result.is_err())
    }

    #[test]
    fn photo_from_val() {
        let val = Value::Array(vec![Value::Object(example_photo_size_val())]);
        let result = <Value as TryInto<Photo>>::try_into(val).unwrap();
        assert_eq!(
            result,
            Photo {
                sizes: vec![example_photo_size_parsed()]
            }
        )
    }

    #[test]
    fn photo_from_val_bad_size() {
        let val = Value::Array(vec![Value::String("bad value".to_owned())]);
        let result = <Value as TryInto<Photo>>::try_into(val);
        assert!(result.is_err())
    }

    #[test]
    fn photo_from_val_wrong_type() {
        let val = Value::String("Bad value".to_owned());
        let result = <Value as TryInto<Photo>>::try_into(val);
        assert!(result.is_err())
    }
}
