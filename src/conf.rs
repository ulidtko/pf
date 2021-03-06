use crate::constants::*;
use crate::utils;

pub fn config_log() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(fern::log_file(utils::home_dir().unwrap().join(LOG_FILE)).unwrap())
        .apply()
        .unwrap();
}
