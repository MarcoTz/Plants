use log::{set_logger, set_max_level, LevelFilter, Log};

pub fn init_logger<T: Log>(logger: &'static T) -> Result<(), String> {
    let _ = set_logger(logger).map_err(|err| format!("{}", err))?;
    let _ = set_max_level(LevelFilter::Trace); //.map_err(|err| format!("{}", err))?;
    Ok(())
}
