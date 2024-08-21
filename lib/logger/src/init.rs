use log::{set_logger, set_max_level, LevelFilter, Log};

pub trait Logger: Log {
    fn setup(&self) -> Result<(), String>;
}

pub fn init_logger<T: Logger>(logger: &'static T) -> Result<(), String> {
    logger.setup()?;
    let _ = set_logger(logger).map_err(|err| format!("{}", err))?;
    let _ = set_max_level(LevelFilter::Trace);
    Ok(())
}
