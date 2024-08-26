pub mod activities;
pub mod growth;
pub mod images;
pub mod plants;
pub mod species;
use crate::errors::Error;

pub trait Port<U>: Sized {
    type LoadArgs;
    type SaveArgs;
    type ConvertArgs;

    fn load_old(args: &Self::LoadArgs) -> Result<Self, Error>;
    fn convert(self, args: &Self::ConvertArgs) -> Result<U, Error>;
    fn save_new(new: U, args: &Self::SaveArgs) -> Result<(), Error>;

    fn port(
        load_args: &Self::LoadArgs,
        convert_args: &Self::ConvertArgs,
        save_args: &Self::SaveArgs,
    ) -> Result<(), Error> {
        let old = Self::load_old(load_args)?;
        let new = old.convert(convert_args)?;
        Self::save_new(new, save_args)?;
        Ok(())
    }
}
