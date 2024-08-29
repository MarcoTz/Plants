use super::{bot::Bot, commands::Command, errors::Error, message::Message};

pub trait MessageHandler {
    type Error: std::error::Error;
    fn handle(&self, bot: &Bot, message: Message) -> Result<(), Self::Error>;

    fn handle_message<'a>(
        &self,
        message: Message,
        bot: &Bot,
    ) -> Result<(), Box<dyn std::error::Error + 'a>>
    where
        Self: 'a,
    {
        if !message.is_command() {
            let handle_res = self.handle(bot, message);
            handle_res.map_err(|err| err.into())
        } else {
            Err(Error::MessageIsCommand(message).into())
        }
    }
}

pub trait CommandHandler<T: Command> {
    type Error: std::error::Error;
    fn handle(&self, bot: &Bot, cmd: T) -> Result<(), Self::Error>;

    fn handle_command<'a>(
        &self,
        message: Message,
        bot: &Bot,
    ) -> Result<(), Box<dyn std::error::Error + 'a>>
    where
        Self: 'a,
        T: 'a,
    {
        let cmd = message.get_command::<T>()?;
        let handle_res = self.handle(bot, cmd);
        handle_res.map_err(|err| err.into())
    }
}

pub trait ErrorHandler<'a> {
    fn handle_error(&self, err: Box<dyn std::error::Error + 'a>);
}
