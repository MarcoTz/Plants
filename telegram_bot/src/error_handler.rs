use bot_api::handlers::ErrorHandler;

pub struct ErrHandler;

impl<'a> ErrorHandler<'a> for ErrHandler {
    fn handle_error(&self, err: Box<dyn std::error::Error + 'a>) {
        println!("{err}")
    }
}
