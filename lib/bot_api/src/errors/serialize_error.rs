use std::fmt;

#[derive(Clone, Debug)]
pub struct SerializeError {
    line: i32,
    column: i32,
    kind: String,
    reason: String,
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
