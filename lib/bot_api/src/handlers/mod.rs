use super::{bot::Bot, commands::Command, message::Message};

pub trait Handler<T: Command> {
    type Error: std::error::Error;
    async fn handle_message(&mut self, bot: &Bot, message: Message) -> Result<(), Self::Error>;
    async fn handle_command(
        &mut self,
        bot: &Bot,
        cmd: T,
        message: Message,
    ) -> Result<(), Self::Error>;

    async fn handle<'a>(
        &mut self,
        message: Message,
        bot: &Bot,
    ) -> Result<(), Box<dyn std::error::Error + 'a>>
    where
        T: 'a,
        Self: 'a,
    {
        if message.is_command() {
            match message.get_command::<T>() {
                Ok(cmd) => {
                    self.handle_command(bot, cmd, message).await?;
                }
                Err(err) => {
                    log::error!("{err}");
                }
            }
            Ok(())
        } else {
            self.handle_message(bot, message).await?;
            Ok(())
        }
    }
}
