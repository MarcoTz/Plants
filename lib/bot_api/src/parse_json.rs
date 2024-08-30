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
