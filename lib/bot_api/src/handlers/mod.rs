use super::{bot::Bot, commands::Command, message::Message, photo_size::Photo};
use std::future::Future;

pub trait Handler<T: Command> {
    fn handle_msg(&mut self, b: &mut Bot, msg: Message) -> impl Future<Output = ()>;
    fn handle_cmd(&mut self, b: &mut Bot, cmd: T, msg: Message) -> impl Future<Output = ()>;
    fn handle_img(&mut self, b: &mut Bot, photo: Photo, msg: Message) -> impl Future<Output = ()>;
}
