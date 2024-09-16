use super::errors::{BadResponse, BadValue, Error, WrongType};
use serde_json::{Map, Value};

pub fn get_map(val: Value) -> Result<Map<String, Value>, Error> {
    if let Value::Object(val_map) = val {
        Ok(val_map)
    } else {
        Err(WrongType {
            field_name: "Entire Response".to_owned(),
            field_type: "Map".to_owned(),
        }
        .into())
    }
}

pub fn check_ok(val: Value) -> Result<Value, Error> {
    let mut val_map = get_map(val.clone())?;

    let ok_val = get_val(&mut val_map, "ok")?;

    if let Value::Bool(true) = ok_val {
        val_map
            .remove("result")
            .ok_or(BadResponse::MissingField("result".to_owned()).into())
    } else {
        Err(BadValue {
            field: "ok".to_owned(),
            val: ok_val.to_owned(),
        }
        .into())
    }
}

pub fn get_val(val_map: &mut Map<String, Value>, field_name: &str) -> Result<Value, Error> {
    val_map
        .remove(field_name)
        .ok_or(BadResponse::MissingField(field_name.to_owned()).into())
}

pub fn get_array(val_map: &mut Map<String, Value>, field_name: &str) -> Result<Vec<Value>, Error> {
    let arr_val = get_val(val_map, field_name)?;
    if let Value::Array(vals) = arr_val {
        Ok(vals)
    } else {
        Err(BadValue {
            val: arr_val,
            field: field_name.to_owned(),
        }
        .into())
    }
}

pub fn get_option_array(
    val_map: &mut Map<String, Value>,
    field_name: &str,
) -> Result<Option<Vec<Value>>, Error> {
    match val_map.remove(field_name) {
        None => Ok(None),
        Some(Value::Array(vals)) => Ok(Some(vals)),
        Some(val) => Err(BadValue {
            val,
            field: field_name.to_owned(),
        }
        .into()),
    }
}

pub fn get_i64(val_map: &mut Map<String, Value>, field_name: &str) -> Result<i64, Error> {
    let field_val = get_val(val_map, field_name)?;
    if let Value::Number(num_val) = field_val {
        let int_val = num_val.as_i64().ok_or(WrongType {
            field_name: field_name.to_owned(),
            field_type: "i64".to_owned(),
        })?;
        Ok(int_val)
    } else {
        Err(BadValue {
            field: field_name.to_owned(),
            val: field_val,
        }
        .into())
    }
}

pub fn get_option_i64(
    val_map: &mut Map<String, Value>,
    field_name: &str,
) -> Result<Option<i64>, Error> {
    match val_map.remove(field_name) {
        None => Ok(None),
        Some(Value::Number(num_val)) => {
            let int = num_val.as_i64().ok_or(WrongType {
                field_name: field_name.to_owned(),
                field_type: "i64".to_owned(),
            })?;
            Ok(Some(int))
        }
        Some(val) => Err(BadValue {
            field: field_name.to_owned(),
            val,
        }
        .into()),
    }
}

pub fn get_option_str(
    val_map: &mut Map<String, Value>,
    field_name: &str,
) -> Result<Option<String>, Error> {
    match val_map.remove(field_name) {
        None => Ok(None),
        Some(Value::String(str_val)) => Ok(Some(str_val)),
        Some(val) => Err(BadValue {
            field: field_name.to_owned(),
            val,
        }
        .into()),
    }
}
pub fn get_str(val_map: &mut Map<String, Value>, field_name: &str) -> Result<String, Error> {
    let field_val = get_val(val_map, field_name)?;
    if let Value::String(str_val) = field_val {
        Ok(str_val)
    } else {
        Err(BadValue {
            field: field_name.to_owned(),
            val: field_val,
        }
        .into())
    }
}
pub fn get_bool(val_map: &mut Map<String, Value>, field_name: &str) -> Result<bool, Error> {
    let field_val = get_val(val_map, field_name)?;
    if let Value::Bool(b) = field_val {
        Ok(b)
    } else {
        Err(BadValue {
            field: field_name.to_owned(),
            val: field_val,
        }
        .into())
    }
}
pub fn get_option_bool(
    val_map: &mut Map<String, Value>,
    field_name: &str,
) -> Result<Option<bool>, Error> {
    match val_map.remove(field_name) {
        None => Ok(None),
        Some(Value::Bool(b)) => Ok(Some(b)),
        Some(val) => Err(BadValue {
            field: field_name.to_owned(),
            val,
        }
        .into()),
    }
}

pub fn get_filename(val: Value) -> Result<String, Error> {
    match val {
        Value::Object(map) => {
            let val_map =
                map.get("result")
                    .ok_or(Error::BadResponse(BadResponse::MissingField(
                        "result".to_owned(),
                    )))?;
            let path =
                val_map
                    .get("file_path")
                    .ok_or(Error::BadResponse(BadResponse::MissingField(
                        "file_path".to_owned(),
                    )))?;
            if let Value::String(st) = path {
                Ok(st.to_owned())
            } else {
                Err(WrongType {
                    field_name: "file_path".to_owned(),
                    field_type: "string".to_owned(),
                }
                .into())
            }
        }
        _ => Err(BadResponse::MissingField("result".to_owned()).into()),
    }
}

#[cfg(test)]
mod parse_json_tests {
    use super::{
        check_ok, get_array, get_bool, get_i64, get_map, get_option_array, get_option_bool,
        get_option_i64, get_option_str, get_str, get_val,
    };
    use serde_json::{Map, Number, Value};

    #[test]
    fn map_value() {
        let result = get_map(Value::Object(Map::new())).unwrap();
        assert_eq!(result, Map::new())
    }

    #[test]
    fn map_value_fail() {
        let result = get_map(Value::String("bad map".to_owned()));
        assert!(result.is_err())
    }

    #[test]
    fn check_ok_ok() {
        let mut val_map = Map::new();
        val_map.insert("ok".to_owned(), Value::Bool(true));
        val_map.insert("result".to_owned(), Value::Bool(true));
        let result = check_ok(Value::Object(val_map)).unwrap();
        assert_eq!(result, Value::Bool(true))
    }

    #[test]
    fn check_ok_no_map() {
        let result = check_ok(Value::Bool(false));
        assert!(result.is_err())
    }

    #[test]
    fn check_ok_no_ok() {
        let result = check_ok(Value::Object(Map::new()));
        assert!(result.is_err())
    }

    #[test]
    fn check_ok_no_result() {
        let mut val_map = Map::new();
        val_map.insert("ok".to_owned(), Value::Bool(true));
        let result = check_ok(Value::Object(val_map));
        assert!(result.is_err())
    }

    #[test]
    fn check_ok_no_bool() {
        let mut val_map = Map::new();
        val_map.insert("ok".to_owned(), Value::String("bad value".to_owned()));
        let result = check_ok(Value::Object(val_map));
        assert!(result.is_err())
    }

    #[test]
    fn get_val_ok() {
        let mut val_map = Map::new();
        val_map.insert("field".to_owned(), Value::Bool(true));
        let result = get_val(&mut val_map, "field").unwrap();
        assert_eq!(result, Value::Bool(true))
    }

    #[test]
    fn get_val_missing() {
        let result = get_val(&mut Map::new(), "something");
        assert!(result.is_err())
    }

    #[test]
    fn get_array_ok() {
        let mut val_map = Map::new();
        val_map.insert("array".to_owned(), Value::Array(vec![Value::Bool(true)]));
        let result = get_array(&mut val_map, "array").unwrap();
        assert_eq!(result, vec![Value::Bool(true)])
    }

    #[test]
    fn get_array_no_field() {
        let result = get_array(&mut Map::new(), "array");
        assert!(result.is_err())
    }

    #[test]
    fn get_array_no_array() {
        let mut val_map = Map::new();
        val_map.insert("array".to_owned(), Value::Bool(false));
        let result = get_array(&mut val_map, "array");
        assert!(result.is_err())
    }

    #[test]
    fn get_option_array_some() {
        let mut val_map = Map::new();
        val_map.insert("array".to_owned(), Value::Array(vec![Value::Bool(true)]));
        let result = get_option_array(&mut val_map, "array").unwrap();
        assert_eq!(result, Some(vec![Value::Bool(true)]))
    }

    #[test]
    fn get_option_array_none() {
        let result = get_option_array(&mut Map::new(), "array").unwrap();
        assert_eq!(result, None)
    }

    #[test]
    fn get_option_array_fail() {
        let mut val_map = Map::new();
        val_map.insert("array".to_owned(), Value::Bool(true));
        let result = get_option_array(&mut val_map, "array");
        assert!(result.is_err())
    }

    #[test]
    fn get_int() {
        let mut val_map = Map::new();
        val_map.insert("int".to_owned(), Value::Number(Number::from(1)));
        let result = get_i64(&mut val_map, "int").unwrap();
        assert_eq!(result, 1)
    }

    #[test]
    fn get_int_no_field() {
        let result = get_i64(&mut Map::new(), "int");
        assert!(result.is_err())
    }

    #[test]
    fn get_int_float() {
        let mut val_map = Map::new();
        let num = Number::from_f64(2.5).unwrap();
        val_map.insert("int".to_owned(), Value::Number(num));
        let result = get_i64(&mut val_map, "int");
        assert!(result.is_err())
    }

    #[test]
    fn get_option_i64_some() {
        let mut val_map = Map::new();
        val_map.insert("int".to_owned(), Value::Number(Number::from(1)));
        let result = get_option_i64(&mut val_map, "int").unwrap();
        assert_eq!(result, Some(1));
    }

    #[test]
    fn get_option_i64_none() {
        let result = get_option_i64(&mut Map::new(), "int").unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn get_option_i64_wrong_type() {
        let mut val_map = Map::new();
        val_map.insert("int".to_owned(), Value::Bool(false));
        let result = get_option_i64(&mut val_map, "int");
        assert!(result.is_err())
    }

    #[test]
    fn get_option_i64_float() {
        let mut val_map = Map::new();
        let num = Number::from_f64(2.4).unwrap();
        val_map.insert("int".to_owned(), Value::Number(num));
        let result = get_option_i64(&mut val_map, "int");
        assert!(result.is_err())
    }

    #[test]
    fn get_str_ok() {
        let mut val_map = Map::new();
        val_map.insert("str".to_owned(), Value::String("str".to_owned()));
        let result = get_str(&mut val_map, "str").unwrap();
        assert_eq!(result, "str")
    }

    #[test]
    fn get_str_missing() {
        let result = get_str(&mut Map::new(), "str");
        assert!(result.is_err())
    }

    #[test]
    fn get_str_wrong_type() {
        let mut val_map = Map::new();
        val_map.insert("str".to_owned(), Value::Bool(false));
        let result = get_str(&mut val_map, "str");
        assert!(result.is_err())
    }

    #[test]
    fn get_option_str_some() {
        let mut val_map = Map::new();
        val_map.insert("str".to_owned(), Value::String("str".to_owned()));
        let result = get_option_str(&mut val_map, "str").unwrap();
        assert_eq!(result, Some("str".to_owned()))
    }

    #[test]
    fn get_option_str_none() {
        let result = get_option_str(&mut Map::new(), "str").unwrap();
        assert_eq!(result, None)
    }

    #[test]
    fn get_option_str_wrong_type() {
        let mut val_map = Map::new();
        val_map.insert("str".to_owned(), Value::Bool(false));
        let result = get_option_str(&mut val_map, "str");
        assert!(result.is_err())
    }

    #[test]
    fn get_bool_ok() {
        let mut val_map = Map::new();
        val_map.insert("bool".to_owned(), Value::Bool(true));
        let result = get_bool(&mut val_map, "bool").unwrap();
        assert!(result)
    }

    #[test]
    fn get_bool_missing() {
        let result = get_bool(&mut Map::new(), "bool");
        assert!(result.is_err())
    }

    #[test]
    fn get_bool_wrong_type() {
        let mut val_map = Map::new();
        val_map.insert("bool".to_owned(), Value::String("bad value".to_owned()));
        let result = get_bool(&mut val_map, "bool");
        assert!(result.is_err())
    }

    #[test]
    fn get_option_bool_some() {
        let mut val_map = Map::new();
        val_map.insert("bool".to_owned(), Value::Bool(true));
        let result = get_option_bool(&mut val_map, "bool").unwrap();
        assert_eq!(result, Some(true))
    }

    #[test]
    fn get_option_bool_none() {
        let result = get_option_bool(&mut Map::new(), "bool").unwrap();
        assert_eq!(result, None)
    }

    #[test]
    fn get_option_bool_wrong_type() {
        let mut val_map = Map::new();
        val_map.insert("bool".to_owned(), Value::String("bad value".to_owned()));
        let result = get_option_bool(&mut val_map, "bool");
        assert!(result.is_err())
    }
}
