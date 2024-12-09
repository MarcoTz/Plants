pub mod bot;
pub mod bot_methods;
pub mod chat;
pub mod commands;
pub mod errors;
pub mod handlers;
pub mod message;
pub mod parse_json;
pub mod photo_size;
pub mod update;
pub mod user;

#[cfg(test)]
pub mod test_common {
    use crate::{bot::Bot, commands::Command, handlers::Handler, message::Message};
    use serde::Deserialize;
    use serde_json::from_str;
    use std::{fmt, fs, path::PathBuf};

    #[derive(Deserialize)]
    pub struct JSONData {
        pub api_key: String,
        pub white_list: Vec<i64>,
    }

    pub fn load_config() -> JSONData {
        let config_path = PathBuf::from("../../testing/bot_conf.json");
        let file_contents = fs::read_to_string(config_path).unwrap();
        let res: JSONData = from_str(&file_contents).unwrap();
        res
    }

    #[test]
    fn load_config_test() {
        load_config();
    }

    pub struct ExampleHandler;
    #[derive(Debug, PartialEq, Eq)]
    pub enum ExampleCommand {
        Succ,
        Error,
    }
    #[derive(Debug)]
    pub struct ExampleError;
    impl fmt::Display for ExampleError {
        fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
            frmt.write_str("example")
        }
    }

    impl std::error::Error for ExampleError {}

    #[test]
    fn display_example_err() {
        let result = format!("{}", ExampleError {});
        let expected = "example";
        assert_eq!(result, expected)
    }

    impl Command for ExampleCommand {
        fn parse(s: &str) -> Result<ExampleCommand, Box<dyn std::error::Error>> {
            match s {
                "succ" => Ok(ExampleCommand::Succ),
                "err" => Ok(ExampleCommand::Error),
                _ => Err(Box::new(ExampleError)),
            }
        }
        fn get_description(&self) -> String {
            panic!("not implemented")
        }
    }

    #[test]
    #[should_panic]
    fn example_command_description() {
        ExampleCommand::Succ.get_description();
    }

    impl Handler<ExampleCommand> for ExampleHandler {
        async fn handle_msg(&mut self, _: &mut Bot, _: Message) {
            return ();
        }
        async fn handle_cmd(&mut self, _: &mut Bot, _: ExampleCommand, _: Message) {
            return ();
        }
        async fn handle_img(&mut self, _: &mut Bot, _: super::photo_size::Photo, _: Message) {
            panic!("not implemented")
        }
    }
}
