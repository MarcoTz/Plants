use std::fmt;

#[derive(Clone, Debug)]
pub struct SerializeError {
    pub line: i32,
    pub column: i32,
    pub kind: String,
    pub reason: String,
}

impl fmt::Display for SerializeError {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        let msg = format!(
            "Could not deserialize\n Error io kind: {}, Category: {} at line {}, column {}",
            self.kind, self.reason, self.line, self.column
        );
        frmt.write_str(&msg)
    }
}

impl From<serde_json::Error> for SerializeError {
    fn from(serde_err: serde_json::Error) -> SerializeError {
        SerializeError {
            line: serde_err.line() as i32,
            column: serde_err.column() as i32,
            reason: format!("{:?}", serde_err.classify()),
            kind: format!("{:?}", serde_err.io_error_kind()),
        }
    }
}

#[cfg(test)]
mod serialize_err_tests {
    use super::SerializeError;

    #[test]
    fn display_serialize_err() {
        let result = format!(
            "{}",
            SerializeError {
                line: 1,
                column: 1,
                kind: "testing".to_owned(),
                reason: "testing".to_owned()
            }
        );
        let expected =
            "Could not deserialize\n Error io kind: testing, Category: testing at line 1, column 1";
        assert_eq!(result, expected)
    }
}
