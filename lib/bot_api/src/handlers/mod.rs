use super::{bot::Bot, commands::Command, errors::Error, message::Message};

pub trait MessageHandler {
    type Error: std::error::Error;
    async fn handle(&mut self, bot: &Bot, message: Message) -> Result<(), Self::Error>;

    async fn handle_message<'a>(
        &mut self,
        message: Message,
        bot: &Bot,
    ) -> Result<(), Box<dyn std::error::Error + 'a>>
    where
        Self: 'a,
    {
        if !message.is_command() {
            let handle_res = self.handle(bot, message).await;
            handle_res.map_err(|err| err.into())
        } else {
            Err(Error::MessageIsCommand(Box::new(message)).into())
        }
    }
}

pub trait CommandHandler<T: Command> {
    type Error: std::error::Error;
    async fn handle(&mut self, bot: &Bot, cmd: T, message: Message) -> Result<(), Self::Error>;

    async fn handle_command<'a>(
        &mut self,
        message: Message,
        bot: &Bot,
    ) -> Result<(), Box<dyn std::error::Error + 'a>>
    where
        Self: 'a,
        T: 'a,
    {
        let cmd = message.get_command::<T>()?;
        let handle_res = self.handle(bot, cmd, message).await;
        handle_res.map_err(|err| err.into())
    }
}

pub trait ErrorHandler<'a> {
    fn handle_error(&self, err: Box<dyn std::error::Error + 'a>);
}
